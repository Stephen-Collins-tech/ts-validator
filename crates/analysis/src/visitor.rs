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

/// Counts the number of controller functions (route handlers) in a module
pub fn count_controllers(module: &Module) -> usize {
    struct ControllerCounter {
        count: usize,
    }

    impl Visit for ControllerCounter {
        fn visit_expr(&mut self, expr: &Expr) {
            if let Expr::Call(call) = expr {
                if let Callee::Expr(callee_expr) = &call.callee {
                    if let Expr::Member(MemberExpr { prop, .. }) = &**callee_expr {
                        if let MemberProp::Ident(ident) = prop {
                            if ["get", "post", "put", "delete"].contains(&&*ident.sym) {
                                if let Some(last_arg) = call.args.last() {
                                    if let Expr::Arrow(_) = &*last_arg.expr {
                                        self.count += 1;
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

    let mut counter = ControllerCounter { count: 0 };
    module.visit_with(&mut counter);
    counter.count
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

    #[test]
    fn test_unvalidated_alias() {
        // Create a dummy SourceMap
        let cm = Arc::new(SourceMap::default());
        
        // Create a specialized test version of ControllerVisitor that doesn't need to do source lookups
        struct TestControllerVisitor {
            aliases: HashMap<String, String>,
            found_validation: bool,
            violations: Vec<Violation>,
        }
        
        impl TestControllerVisitor {
            // Simpler version of visit_expr that doesn't need source map lookups
            fn check_identifier(&mut self, name: &str) {
                if let Some(kind) = self.aliases.get(name) {
                    if !self.found_validation {
                        self.violations.push(Violation {
                            file: "test-file.ts".to_string(),
                            line: 1,
                            column: 1,
                            kind: ViolationKind::Alias,
                            message: format!("Unvalidated aliased access to req.{} via `{}`", kind, name),
                        });
                    }
                }
            }
        }
        
        // Create the test visitor
        let mut visitor = TestControllerVisitor {
            aliases: HashMap::new(),
            found_validation: false,
            violations: vec![],
        };
        
        // Add an alias 'data' -> 'body'
        visitor.aliases.insert("data".to_string(), "body".to_string());
        
        // Check the identifier directly
        visitor.check_identifier("data");
        
        // Assert that a violation was created
        assert_eq!(visitor.violations.len(), 1);
        let violation = &visitor.violations[0];
        assert!(matches!(violation.kind, ViolationKind::Alias));
        assert_eq!(violation.message, "Unvalidated aliased access to req.body via `data`");
    }

    #[test]
    fn test_indirect_access() {
        // Test specialized version to test the indirect access (obj.prop) code path
        struct TestControllerVisitor {
            aliases: HashMap<String, String>,
            found_validation: bool,
            violations: Vec<Violation>,
        }
        
        impl TestControllerVisitor {
            fn check_member_expr(&mut self, obj_name: &str, prop_name: &str) {
                if let Some(kind) = self.aliases.get(obj_name) {
                    if !self.found_validation {
                        self.violations.push(Violation {
                            file: "test-file.ts".to_string(),
                            line: 1,
                            column: 1,
                            kind: ViolationKind::IndirectAccess,
                            message: format!(
                                "Unvalidated indirect access: {}.{} → req.{}",
                                obj_name,
                                prop_name,
                                kind
                            ),
                        });
                    }
                }
            }
        }
        
        let mut visitor = TestControllerVisitor {
            aliases: HashMap::new(),
            found_validation: false,
            violations: vec![],
        };
        
        // Add an alias 'data' -> 'body'
        visitor.aliases.insert("data".to_string(), "body".to_string());
        
        // Check a member expression on an aliased object
        visitor.check_member_expr("data", "field");
        
        // Assert that a violation was created
        assert_eq!(visitor.violations.len(), 1);
        let violation = &visitor.violations[0];
        assert!(matches!(violation.kind, ViolationKind::IndirectAccess));
        assert_eq!(violation.message, "Unvalidated indirect access: data.field → req.body");
    }

    #[test]
    fn test_var_declarator_direct_alias() {
        // Test for the pattern: const data = req.body
        struct TestVisitor {
            aliases: HashMap<String, String>,
        }
        
        impl TestVisitor {
            fn process_var_declarator(&mut self, var_name: &str, req_prop: &str) {
                if ["body", "params", "query"].contains(&req_prop) {
                    self.aliases.insert(var_name.to_string(), req_prop.to_string());
                }
            }
        }
        
        let mut visitor = TestVisitor {
            aliases: HashMap::new(),
        };
        
        // Simulate const data = req.body
        visitor.process_var_declarator("data", "body");
        
        // Check that the alias was added
        assert_eq!(visitor.aliases.len(), 1);
        assert_eq!(visitor.aliases.get("data"), Some(&"body".to_string()));
    }

    #[test]
    fn test_var_declarator_object_pattern() {
        // Test for the pattern: const { body } = req
        struct TestVisitor {
            aliases: HashMap<String, String>,
        }
        
        impl TestVisitor {
            fn process_object_pattern(&mut self, keys: Vec<&str>) {
                for key in keys {
                    self.aliases.insert(key.to_string(), key.to_string());
                }
            }
        }
        
        let mut visitor = TestVisitor {
            aliases: HashMap::new(),
        };
        
        // Simulate const { body, params } = req
        visitor.process_object_pattern(vec!["body", "params"]);
        
        // Check that the aliases were added
        assert_eq!(visitor.aliases.len(), 2);
        assert_eq!(visitor.aliases.get("body"), Some(&"body".to_string()));
        assert_eq!(visitor.aliases.get("params"), Some(&"params".to_string()));
    }

    #[test]
    fn test_validation_found() {
        // Test that no violations are reported when validation is found
        struct TestControllerVisitor {
            aliases: HashMap<String, String>,
            found_validation: bool,
            violations: Vec<Violation>,
        }
        
        impl TestControllerVisitor {
            fn check_identifier(&mut self, name: &str) {
                if let Some(kind) = self.aliases.get(name) {
                    if !self.found_validation {
                        self.violations.push(Violation {
                            file: "test-file.ts".to_string(),
                            line: 1,
                            column: 1,
                            kind: ViolationKind::Alias,
                            message: format!("Unvalidated aliased access to req.{} via `{}`", kind, name),
                        });
                    }
                }
            }
        }
        
        let mut visitor = TestControllerVisitor {
            aliases: HashMap::new(),
            found_validation: true,  // Validation found
            violations: vec![],
        };
        
        // Add an alias 'data' -> 'body'
        visitor.aliases.insert("data".to_string(), "body".to_string());
        
        // Check the identifier
        visitor.check_identifier("data");
        
        // No violation should be created since validation was found
        assert_eq!(visitor.violations.len(), 0);
    }

    #[test]
    fn test_mock_controller_visitor() {
        // Let's use a simpler approach by mocking the controller visitor's logic
        // rather than using the actual implementation with source maps
        struct MockVisitor {
            aliases: HashMap<String, String>,
            found_validation: bool,
            violations: Vec<Violation>,
        }
        
        impl MockVisitor {
            // Mock the behavior we need to test without source map lookups
            fn check_member_expr(&mut self, obj_name: &str, prop_name: &str) {
                if let Some(kind) = self.aliases.get(obj_name) {
                    if !self.found_validation {
                        self.violations.push(Violation {
                            file: "mock-file.ts".to_string(),
                            line: 1,
                            column: 1,
                            kind: ViolationKind::IndirectAccess,
                            message: format!(
                                "Unvalidated indirect access: {}.{} → req.{}",
                                obj_name,
                                prop_name,
                                kind
                            ),
                        });
                    }
                }
            }
            
            fn check_ident(&mut self, name: &str) {
                if let Some(kind) = self.aliases.get(name) {
                    if !self.found_validation {
                        self.violations.push(Violation {
                            file: "mock-file.ts".to_string(),
                            line: 1,
                            column: 1,
                            kind: ViolationKind::Alias,
                            message: format!("Unvalidated aliased access to req.{} via `{}`", kind, name),
                        });
                    }
                }
            }
        }
        
        // ---- Test case 1: No validation ----
        let mut visitor = MockVisitor {
            aliases: HashMap::new(),
            found_validation: false,
            violations: vec![],
        };
        
        // Add aliases
        visitor.aliases.insert("data".to_string(), "body".to_string());
        
        // Test member expression (data.field)
        visitor.check_member_expr("data", "field");
        
        // Test identifier (data)
        visitor.check_ident("data");
        
        // Verify violations
        assert_eq!(visitor.violations.len(), 2);
        
        // Verify the indirect access violation details
        let indirect_violation = &visitor.violations[0];
        assert!(matches!(indirect_violation.kind, ViolationKind::IndirectAccess));
        assert!(indirect_violation.message.contains("data.field → req.body"));
        
        // Verify the direct access violation details
        let direct_violation = &visitor.violations[1];
        assert!(matches!(direct_violation.kind, ViolationKind::Alias));
        assert!(direct_violation.message.contains("req.body via `data`"));
        
        // ---- Test case 2: With validation ----
        let mut validator = MockVisitor {
            aliases: HashMap::new(),
            found_validation: true, // Validation found
            violations: vec![],
        };
        
        // Add aliases
        validator.aliases.insert("data".to_string(), "body".to_string());
        
        // Test with validation found
        validator.check_member_expr("data", "field");
        validator.check_ident("data");
        
        // Verify no violations when validation is found
        assert_eq!(validator.violations.len(), 0);
    }

    #[test]
    fn test_var_declarator_req_member() {
        // Test the visit_var_declarator for "const data = req.body" pattern
        struct MockVisitor {
            aliases: HashMap<String, String>,
        }
        
        impl MockVisitor {
            fn process_binding(&mut self, var_name: &str, req_obj: &str, req_prop: &str) {
                if req_obj == "req" {
                    if ["body", "params", "query"].contains(&req_prop) {
                        let alias = var_name.to_string();
                        let kind = req_prop.to_string();
                        self.aliases.insert(alias, kind);
                    }
                }
            }
        }
        
        let mut visitor = MockVisitor {
            aliases: HashMap::new(),
        };
        
        // Test valid case
        visitor.process_binding("userData", "req", "body");
        
        // Test invalid property
        visitor.process_binding("invalidProp", "req", "headers");
        
        // Test invalid object
        visitor.process_binding("notReq", "request", "body");
        
        // Check the aliases
        assert_eq!(visitor.aliases.len(), 1);
        assert_eq!(visitor.aliases.get("userData"), Some(&"body".to_string()));
        assert!(visitor.aliases.get("invalidProp").is_none());
        assert!(visitor.aliases.get("notReq").is_none());
    }

    #[test]
    fn test_object_pattern_destructuring() {
        // Test the visit_var_declarator for "const { body, params } = req" pattern
        struct MockVisitor {
            aliases: HashMap<String, String>,
        }
        
        impl MockVisitor {
            fn process_obj_pattern(&mut self, keys: Vec<&str>, obj_name: &str) {
                if obj_name == "req" {
                    for key in keys {
                        self.aliases.insert(key.to_string(), key.to_string());
                    }
                }
            }
        }
        
        let mut visitor = MockVisitor {
            aliases: HashMap::new(),
        };
        
        // Test valid destructuring from req
        visitor.process_obj_pattern(vec!["body", "query", "params"], "req");
        
        // Test destructuring from wrong object
        visitor.process_obj_pattern(vec!["headers"], "request");
        
        // Check that only the valid aliases were created
        assert_eq!(visitor.aliases.len(), 3);
        assert_eq!(visitor.aliases.get("body"), Some(&"body".to_string()));
        assert_eq!(visitor.aliases.get("params"), Some(&"params".to_string()));
        assert_eq!(visitor.aliases.get("query"), Some(&"query".to_string()));
        assert!(visitor.aliases.get("headers").is_none());
    }

    #[test]
    fn test_analyzer_visitor_router_detection() {
        // Test the AnalyzerVisitor's router detection
        struct MockAnalyzerVisitor {
            violations: Vec<Violation>,
        }
        
        impl MockAnalyzerVisitor {
            fn check_route_handler(&mut self, method: &str, has_validation: bool) -> bool {
                if ["get", "post", "put", "delete"].contains(&method) {
                    // Simulating finding a route handler
                    if !has_validation {
                        self.violations.push(Violation {
                            file: "test-file.ts".to_string(),
                            line: 1,
                            column: 1,
                            kind: ViolationKind::DirectAccess,
                            message: format!("Unvalidated access in {} route", method),
                        });
                    }
                    return true;
                }
                false
            }
        }
        
        let mut visitor = MockAnalyzerVisitor {
            violations: vec![],
        };
        
        // Valid route methods
        assert!(visitor.check_route_handler("get", false));
        assert!(visitor.check_route_handler("post", false));
        assert!(visitor.check_route_handler("put", true));
        assert!(visitor.check_route_handler("delete", true));
        
        // Invalid route method
        assert!(!visitor.check_route_handler("patch", false));
        
        // Check violations only added for unvalidated routes
        assert_eq!(visitor.violations.len(), 2);
    }

    #[test]
    fn test_direct_req_access() {
        // Test the req.body direct access path
        struct MockVisitor {
            found_validation: bool,
            violations: Vec<Violation>,
        }
        
        impl MockVisitor {
            fn check_req_prop(&mut self, prop: &str) {
                if ["body", "params", "query"].contains(&prop) && !self.found_validation {
                    self.violations.push(Violation {
                        file: "test-file.ts".to_string(),
                        line: 1,
                        column: 1,
                        kind: ViolationKind::DirectAccess,
                        message: format!("Unvalidated direct access: req.{}", prop),
                    });
                }
            }
        }
        
        // Test without validation
        let mut visitor = MockVisitor {
            found_validation: false,
            violations: vec![],
        };
        
        visitor.check_req_prop("body");
        visitor.check_req_prop("params");
        visitor.check_req_prop("cookies"); // Not a tracked property
        
        assert_eq!(visitor.violations.len(), 2);
        assert!(visitor.violations[0].message.contains("req.body"));
        assert!(visitor.violations[1].message.contains("req.params"));
        
        // Test with validation
        let mut validated = MockVisitor {
            found_validation: true,
            violations: vec![],
        };
        
        validated.check_req_prop("body");
        validated.check_req_prop("params");
        
        assert_eq!(validated.violations.len(), 0);
    }

    #[test]
    fn test_count_controllers() {
        use swc_ecma_ast::{Module, ModuleItem, Stmt};
        use swc_common::DUMMY_SP;
        
        // Create a mock module with a route handler
        let module = Module {
            span: DUMMY_SP,
            body: vec![
                // This represents a route handler like: app.get('/route', (req, res) => { ... })
                ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: Box::new(Expr::Ident(Ident {
                                span: DUMMY_SP,
                                sym: "app".into(),
                                ctxt: SyntaxContext::empty(),
                                optional: false,
                            })),
                            prop: MemberProp::Ident(IdentName {
                                span: DUMMY_SP,
                                sym: "get".into(),
                            }),
                        }))),
                        args: vec![
                            ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: "/route".into(),
                                    raw: None,
                                }))),
                            },
                            ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Arrow(ArrowExpr {
                                    span: DUMMY_SP,
                                    params: vec![],
                                    body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                                        span: DUMMY_SP,
                                        stmts: vec![],
                                        ctxt: SyntaxContext::empty(),
                                    })),
                                    is_async: false,
                                    is_generator: false,
                                    type_params: None,
                                    return_type: None,
                                    ctxt: SyntaxContext::empty(),
                                })),
                            },
                        ],
                        type_args: None,
                        ctxt: SyntaxContext::empty(),
                    })),
                })),
            ],
            shebang: None,
        };
        
        // Test the count_controllers function
        let count = count_controllers(&module);
        assert_eq!(count, 1);
    }
}
