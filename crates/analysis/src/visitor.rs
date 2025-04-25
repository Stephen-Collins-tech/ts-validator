use std::collections::HashMap;
use std::sync::Arc;

use swc_common::{SourceMap, Spanned};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};
use validation::ValidationRuleSet;
use reporting::violation::{Violation, ViolationKind};

pub struct AnalyzerVisitor<'a> {
    pub cm: &'a Arc<SourceMap>,
    pub violations: Vec<Violation>,
    pub rules: ValidationRuleSet,
}

pub struct ControllerVisitor<'a> {
    pub cm: &'a Arc<SourceMap>,
    pub aliases: HashMap<String, String>,
    pub found_validation: bool,
    pub violations: Vec<Violation>,
    pub rules: ValidationRuleSet,
}

fn member_name(prop: &MemberProp) -> String {
    match prop {
        MemberProp::Ident(ident) => ident.sym.to_string(),
        MemberProp::Computed(comp) => match &*comp.expr {
            Expr::Lit(lit) => format!("[computed: {:?}]", lit),
            Expr::Ident(id) => format!("[computed: {}]", id.sym),
            _ => "[computed]".to_string(),
        },
        _ => "[unsupported]".to_string(),
    }
}

impl<'a> Visit for AnalyzerVisitor<'a> {
    fn visit_expr(&mut self, expr: &Expr) {
        if let Expr::Call(call) = expr {
            if let Callee::Expr(callee_expr) = &call.callee {
                if let Expr::Member(MemberExpr { prop, .. }) = &**callee_expr {
                    if let MemberProp::Ident(ident) = prop {
                        if ["get", "post", "put", "delete"].contains(&&*ident.sym) {
                            if let Some(last_arg) = call.args.last() {
                                if let Expr::Arrow(arrow_fn) = &*last_arg.expr {
                                    println!("📦 Found controller for .{}()", ident.sym);

                                    let mut controller = ControllerVisitor {
                                        cm: self.cm,
                                        aliases: HashMap::new(),
                                        found_validation: false,
                                        violations: vec![],
                                        rules: self.rules,
                                    };

                                    arrow_fn.body.visit_with(&mut controller);
                                    self.violations.extend(controller.violations);
                                }
                            }
                        }
                    }
                }
            }
        }

        expr.visit_children_with(self);
    }
}

impl<'a> Visit for ControllerVisitor<'a> {
    fn visit_expr(&mut self, expr: &Expr) {
        if validation::is_validation_call(expr, self.rules) {
            self.found_validation = true;
        }

        if let Expr::Member(MemberExpr { obj, prop, .. }) = expr {
            if let Expr::Ident(obj_ident) = &**obj {
                let loc = self.cm.lookup_char_pos(expr.span().lo());

                if obj_ident.sym == *"req" {
                    if let MemberProp::Ident(prop_ident) = prop {
                        if ["body", "params", "query"].contains(&&*prop_ident.sym) && !self.found_validation {
                            self.violations.push(Violation {
                                file: loc.file.name.to_string(),
                                line: loc.line,
                                column: loc.col_display,
                                kind: ViolationKind::DirectAccess,
                                message: format!("Unvalidated direct access: req.{}", prop_ident.sym),
                            });
                        }
                    }
                } else if let Some(kind) = self.aliases.get(&obj_ident.sym.to_string()) {
                    if !self.found_validation {
                        self.violations.push(Violation {
                            file: loc.file.name.to_string(),
                            line: loc.line,
                            column: loc.col_display,
                            kind: ViolationKind::IndirectAccess,
                            message: format!(
                                "Unvalidated indirect access: {}.{} → req.{}",
                                obj_ident.sym,
                                member_name(prop),
                                kind
                            ),
                        });
                    }
                }
            }
        }

        if let Expr::Ident(id) = expr {
            if let Some(kind) = self.aliases.get(&id.sym.to_string()) {
                if !self.found_validation {
                    let loc = self.cm.lookup_char_pos(expr.span().lo());
                    self.violations.push(Violation {
                        file: loc.file.name.to_string(),
                        line: loc.line,
                        column: loc.col_display,
                        kind: ViolationKind::Alias,
                        message: format!("Unvalidated aliased access to req.{} via `{}`", kind, id.sym),
                    });
                }
            }
        }

        expr.visit_children_with(self);
    }

    fn visit_var_declarator(&mut self, decl: &VarDeclarator) {
        use swc_ecma_ast::{Pat::Ident as PatIdent, Pat::Object as PatObject};

        if let PatIdent(binding) = &decl.name {
            if let Some(init) = &decl.init {
                if let Expr::Member(MemberExpr { obj, prop, .. }) = &**init {
                    if let Expr::Ident(obj_ident) = &**obj {
                        if obj_ident.sym == *"req" {
                            if let MemberProp::Ident(prop_ident) = prop {
                                if ["body", "params", "query"].contains(&&*prop_ident.sym) {
                                    let alias = binding.id.sym.to_string();
                                    let kind = prop_ident.sym.to_string();
                                    self.aliases.insert(alias, kind);
                                }
                            }
                        }
                    }
                }
            }
        }

        if let PatObject(obj_pat) = &decl.name {
            if let Some(init) = &decl.init {
                if let Expr::Ident(init_ident) = &**init {
                    if init_ident.sym == *"req" {
                        for prop in &obj_pat.props {
                            if let ObjectPatProp::KeyValue(kv) = prop {
                                if let PropName::Ident(key_ident) = &kv.key {
                                    let alias = key_ident.sym.to_string();
                                    self.aliases.insert(alias.clone(), alias);
                                }
                            }
                        }
                    }
                }
            }
        }

        decl.visit_children_with(self);
    }
}

pub fn visit_module(cm: &Arc<SourceMap>, module: &Module, rules: ValidationRuleSet) -> Vec<Violation> {
    let mut visitor = AnalyzerVisitor {
        cm,
        violations: vec![],
        rules,
    };
    visitor.visit_module(module);
    visitor.violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_common::{DUMMY_SP, BytePos, SyntaxContext};
    use swc_ecma_ast::{Ident, IdentName, ComputedPropName, Expr, Lit, Str};

    #[test]
    fn test_member_name_ident() {
        let ident_name = IdentName { span: DUMMY_SP, sym: "testProp".into() };
        let prop = MemberProp::Ident(ident_name);
        assert_eq!(member_name(&prop), "testProp");
    }

    #[test]
    fn test_member_name_computed_lit() {
        let prop = MemberProp::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Box::new(Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: "computedProp".into(),
                raw: None,
            }))),
        });
        assert_eq!(member_name(&prop), r#"[computed: Str(Str { span: 0..0, value: "computedProp", raw: None })]"#);
    }

    #[test]
    fn test_member_name_computed_ident() {
        let ident = Ident { span: DUMMY_SP, sym: "varName".into(), ctxt: SyntaxContext::empty(), optional: false };
        let prop = MemberProp::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Box::new(Expr::Ident(ident)),
        });
        assert_eq!(member_name(&prop), "[computed: varName]");
    }

    #[test]
    fn test_member_name_computed_other() {
        let call_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                span: DUMMY_SP,
                sym: "someFunc".into(),
                ctxt: SyntaxContext::empty(),
                optional: false,
            }))),
            args: vec![],
            type_args: None,
            ctxt: SyntaxContext::empty(),
        });
        let prop = MemberProp::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Box::new(call_expr),
        });
        assert_eq!(member_name(&prop), "[computed]");
    }

    #[test]
    fn test_member_name_private_name() {
        let private_name = PrivateName { span: DUMMY_SP, name: "privateField".into() };
        let prop = MemberProp::PrivateName(private_name);
        assert_eq!(member_name(&prop), "[unsupported]");
    }

    // TODO: Add tests for AnalyzerVisitor and ControllerVisitor
}
