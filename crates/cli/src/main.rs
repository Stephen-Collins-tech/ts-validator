use analysis::analyze_modules;
use clap::{Arg, ArgAction, Command};
use parser::parser::parse_dir;
use std::path::Path;
use std::time::Instant;
use validation::ValidationRuleSet;
use serde_json;
use serde::Serialize;
use reporting::violation::Violation;
use utils::logging::{set_json_mode, log, error};

#[derive(Serialize)]
struct AnalysisResult {
    files_parsed: usize,
    violations_count: usize,
    elapsed_time_ms: u128,
    violations: Vec<Violation>,
}

// Extract CLI argument parsing to make it testable
fn build_cli() -> Command {
    Command::new("ts-validator")
        .about("Rust-based TypeScript Runtime Validation Checker")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("path")
                .help("Directory or file to analyze")
                .required(true),
        )
        .arg(
            Arg::new("rules")
                .long("rules")
                .help("Validation rule set to use")
                .default_value("zod-strict")
                .value_parser(["zod-strict", "zod-lenient", "custom"]),
        )
        .arg(
            Arg::new("fail-on-warning")
                .long("fail-on-warning")
                .help("Exit with code 1 if violations are found")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("Output results as JSON")
                .action(ArgAction::SetTrue),
        )
}

fn main() {
    let matches = build_cli().get_matches();

    let entry_path = Path::new(matches.get_one::<String>("path").unwrap());
    let json_mode = matches.get_flag("json");
    let fail_on_warning = matches.get_flag("fail-on-warning");
    
    // Set global JSON mode flag
    set_json_mode(json_mode);

    if !entry_path.exists() {
        error(&format!("Path does not exist: {}", entry_path.display()));
        std::process::exit(1);
    }

    let rules = match matches.get_one::<String>("rules").map(|s| s.as_str()) {
        Some("zod-lenient") => ValidationRuleSet::ZodLenient,
        Some("custom") => ValidationRuleSet::Custom,
        _ => ValidationRuleSet::ZodStrict,
    };
    
    let rule_name = match matches.get_one::<String>("rules") {
        Some(name) => name,
        None => "zod-strict",
    };

    log(&format!("🔍 Starting analysis on {} with rules: {}", entry_path.display(), rule_name));

    let start_time = Instant::now();
    
    log("\nParsing files:");

    let parsed_modules = parse_dir(entry_path);
    let parsed_modules_len = parsed_modules.len(); 
    
    log(&format!("\n🛠  Parsing completed in: {:?}", start_time.elapsed()));
    log("\nAnalyzing files:");
    
    let violations = analyze_modules(parsed_modules, rules);
    let violations_len = violations.len();

    let has_violations = !violations.is_empty();

    if json_mode {
        let result = AnalysisResult {
            files_parsed: parsed_modules_len,
            violations_count: violations_len,
            elapsed_time_ms: start_time.elapsed().as_millis(),
            violations,
        };
        
        // Specifically print the JSON output
        println!("{}", serde_json::to_string(&result).unwrap());
        
        // Exit with code 1 only if --fail-on-warning is set AND violations were found
        if fail_on_warning && has_violations {
            std::process::exit(1);
        }
    } else {
        if violations_len > 0 {
            log("\n❗ violations detected");
        } else {
            log("\n✅ No violations found");
        }

        for v in &violations {
            log(&format!("\x1b[91m[{}:{}:{}] {}\x1b[0m", v.file, v.line, v.column, v.message));
        }
        
        log(&format!(
            "\n🏁 Analysis completed in {:?}",
            start_time.elapsed()
        ));
        log(&format!(
            "✅ {} files parsed | ❗ {} violations found | 🕑 {:?} total",
            parsed_modules_len,
            violations_len,
            start_time.elapsed()
        ));
    }
    
    // This handles the non-JSON mode case
    if fail_on_warning && has_violations {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::logging::is_json_mode;
    use reporting::violation::ViolationKind;
    
    #[test]
    fn test_json_flag_sets_json_mode() {
        // Reset json mode before test
        set_json_mode(false);
        
        // Create arg matches with json flag set to true
        let matches = build_cli()
            .try_get_matches_from(vec!["ts-validator", "--json", "dummy/path"])
            .expect("Failed to parse arguments");
            
        let json_mode = matches.get_flag("json");
        assert!(json_mode, "JSON mode flag should be true");
        
        // Apply the flag to global state
        set_json_mode(json_mode);
        assert!(is_json_mode(), "Global JSON mode should be enabled");
        
        // Reset json mode after test
        set_json_mode(false);
    }
    
    #[test]
    fn test_fail_on_warning_flag() {
        let matches = build_cli()
            .try_get_matches_from(vec!["ts-validator", "--fail-on-warning", "dummy/path"])
            .expect("Failed to parse arguments");
            
        let fail_on_warning = matches.get_flag("fail-on-warning");
        assert!(fail_on_warning, "Fail on warning flag should be true");
    }
    
    #[test]
    fn test_validation_rule_selection() {
        // Test default rule
        let matches = build_cli()
            .try_get_matches_from(vec!["ts-validator", "dummy/path"])
            .expect("Failed to parse arguments");
            
        let rule = matches.get_one::<String>("rules").unwrap();
        assert_eq!(rule, "zod-strict", "Default rule should be zod-strict");
        
        // Test explicit rule selection
        let matches = build_cli()
            .try_get_matches_from(vec!["ts-validator", "--rules", "zod-lenient", "dummy/path"])
            .expect("Failed to parse arguments");
            
        let rule = matches.get_one::<String>("rules").unwrap();
        assert_eq!(rule, "zod-lenient", "Selected rule should be zod-lenient");
    }
    
    #[test]
    fn test_analyze_result_serialization() {
        let result = AnalysisResult {
            files_parsed: 10,
            violations_count: 2,
            elapsed_time_ms: 150,
            violations: vec![
                Violation {
                    file: "test.ts".to_string(),
                    line: 42,
                    column: 10,
                    kind: ViolationKind::DirectAccess,
                    message: "Unvalidated direct access".to_string(),
                }
            ],
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"files_parsed\":10"), "JSON should include files_parsed field");
        assert!(json.contains("\"violations_count\":2"), "JSON should include violations_count field");
        assert!(json.contains("\"elapsed_time_ms\":150"), "JSON should include elapsed_time_ms field");
        assert!(json.contains("\"file\":\"test.ts\""), "JSON should include violation details");
    }
}
