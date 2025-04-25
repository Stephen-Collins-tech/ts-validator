use swc_ecma_ast::{Expr, MemberExpr, Callee};
use swc_ecma_ast::Expr::{Call, Member};

#[derive(Debug, Clone, Copy)]
pub enum ValidationRuleSet {
    ZodStrict,    // Only .parse()
    ZodLenient,   // .parse() or .safeParse()
    Custom,       // Expand as needed
}


/// Returns true if the expression looks like a validation call (e.g., `.parse(...)`)
pub fn is_validation_call(expr: &Expr, rules: ValidationRuleSet) -> bool {
    if let Call(call) = expr {
        if let Callee::Expr(callee_expr) = &call.callee {
            if let Member(MemberExpr { prop, .. }) = &**callee_expr {
                if let swc_ecma_ast::MemberProp::Ident(ident) = prop {
                    return match rules {
                        ValidationRuleSet::ZodStrict => ident.sym == *"parse",
                        ValidationRuleSet::ZodLenient => ident.sym == *"parse" || ident.sym == *"safeParse",
                        ValidationRuleSet::Custom => {
                            // Example: allow `validate` calls
                            ident.sym == *"parse" || ident.sym == *"validate"
                        }
                    };

                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_common::sync::Lrc;
    use swc_common::{FileName, SourceMap};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

    fn parse_expr(code: &str) -> Box<Expr> {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(Lrc::new(FileName::Custom("test.js".into())), code.to_string());
        let lexer = Lexer::new(
            Syntax::Typescript(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module().expect("Failed to parse module");
        // Assuming the expression is the first statement's expression
        match &module.body[0] {
            swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Expr(expr_stmt)) => expr_stmt.expr.clone(),
            _ => panic!("Expected an expression statement"),
        }
    }

    #[test]
    fn test_is_validation_call_zod_strict() {
        let parse_call = parse_expr("schema.parse(data)");
        let safe_parse_call = parse_expr("schema.safeParse(data)");
        let other_call = parse_expr("schema.validate(data)");
        let not_a_call = parse_expr("schema.member");

        assert!(is_validation_call(&parse_call, ValidationRuleSet::ZodStrict));
        assert!(!is_validation_call(&safe_parse_call, ValidationRuleSet::ZodStrict));
        assert!(!is_validation_call(&other_call, ValidationRuleSet::ZodStrict));
        assert!(!is_validation_call(&not_a_call, ValidationRuleSet::ZodStrict));
    }

    #[test]
    fn test_is_validation_call_zod_lenient() {
        let parse_call = parse_expr("schema.parse(data)");
        let safe_parse_call = parse_expr("schema.safeParse(data)");
        let other_call = parse_expr("schema.validate(data)");

        assert!(is_validation_call(&parse_call, ValidationRuleSet::ZodLenient));
        assert!(is_validation_call(&safe_parse_call, ValidationRuleSet::ZodLenient));
        assert!(!is_validation_call(&other_call, ValidationRuleSet::ZodLenient));
    }

     #[test]
    fn test_is_validation_call_custom() {
        let parse_call = parse_expr("schema.parse(data)");
        let safe_parse_call = parse_expr("schema.safeParse(data)");
        let validate_call = parse_expr("schema.validate(data)");
         let other_call = parse_expr("schema.other(data)");


        assert!(is_validation_call(&parse_call, ValidationRuleSet::Custom));
        assert!(!is_validation_call(&safe_parse_call, ValidationRuleSet::Custom));
        assert!(is_validation_call(&validate_call, ValidationRuleSet::Custom));
         assert!(!is_validation_call(&other_call, ValidationRuleSet::Custom));
    }
}
