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
