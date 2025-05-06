//! Library crate for analysis.

mod visitor;

use parser::ParsedModule;
use reporting::violation::Violation;
use std::collections::HashMap;
use validation::ValidationRuleSet;
use visitor::{visit_module, count_controllers};
use utils::logging::log;

/// Analyzes parsed modules and returns a list of violations.
pub fn analyze_modules(modules: Vec<ParsedModule>, rules: ValidationRuleSet) -> Vec<Violation> {
    let mut all_violations = vec![];
    let mut controllers_per_file = HashMap::new();

    // First pass: count controllers for each file
    for parsed in &modules {
        let file_name = parsed.path.iter().rev().take(2).collect::<Vec<_>>().iter().rev().collect::<std::path::PathBuf>();
        let controller_count = count_controllers(&parsed.module);
        if controller_count > 0 {
            controllers_per_file.insert(file_name.to_string_lossy().to_string(), controller_count);
        }
    }

    // Second pass: analyze modules and report with proper format
    for parsed in modules {
        let violations = visit_module(&parsed.source_map, &parsed.module, rules);
        let file_name = parsed.path.iter().rev().take(2).collect::<Vec<_>>().iter().rev().collect::<std::path::PathBuf>();
        let file_path = file_name.to_string_lossy().to_string();
        
        if let Some(controller_count) = controllers_per_file.get(&file_path) {
            log(&format!("📦 {} — {} controllers found", file_name.display(), controller_count));
        } else if violations.is_empty() {
            log(&format!("✅ {} — No violations found", file_name.display()));
        } else {
            log(&format!("❗ {} — Found {} violations", file_name.display(), violations.len()));
        }

        all_violations.extend(violations);
    }

    all_violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use validation::ValidationRuleSet;
    use parser::ParsedModule;
    use swc_common::{sync::Lrc, SourceMap, FileName};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax };
    use std::path::PathBuf;
    use std::sync::Arc;

    // Helper to parse a module (similar to visitor tests)
    fn parse_test_module(cm: &Arc<SourceMap>, code: &str) -> swc_ecma_ast::Module {
        let fm = cm.new_source_file(
            Lrc::new(FileName::Custom("test.ts".into())),
            code.into(),
        );
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                tsx: false,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        parser.parse_module().expect("Failed to parse module")
    }

    #[test]
    fn test_analyze_modules_with_content() {
        let cm = Arc::new(SourceMap::default());
        let module_content = "console.log('hello');";
        let module = parse_test_module(&cm, module_content);

        let parsed_module = ParsedModule {
            path: PathBuf::from("test.ts"),
            source_map: cm.clone(),
            module,
        };

        let modules = vec![parsed_module];
        let rules = ValidationRuleSet::Custom;
        let violations = analyze_modules(modules, rules);
        // Expect no violations for simple console log
        assert!(violations.is_empty());
    }

    #[test]
    fn test_analyze_modules_empty() {
        let modules = vec![];
        let rules = ValidationRuleSet::Custom;
        let violations = analyze_modules(modules, rules);
        assert!(violations.is_empty());
    }
}
