//! Library crate for analysis.

mod visitor;

use parser::ParsedModule;
use reporting::violation::Violation;
use validation::ValidationRuleSet;
use visitor::visit_module;

/// Analyzes parsed modules and returns a list of violations.
pub fn analyze_modules(modules: Vec<ParsedModule>, rules: ValidationRuleSet) -> Vec<Violation> {
    let mut all_violations = vec![];

    for parsed in modules {
        println!("Analyzing: {}", parsed.path.display());

        let violations = visit_module(&parsed.source_map, &parsed.module, rules);
        println!("Found {} violations", violations.len());

        all_violations.extend(violations);
    }

    all_violations
}
