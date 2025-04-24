use analysis::analyze_modules;
use clap::{Arg, ArgAction, Command};
use parser::parser::parse_dir;
use std::path::Path;
use std::time::Instant;
use validation::ValidationRuleSet;

fn main() {
    let matches = Command::new("ts-validator")
        .about("Rust-based TypeScript Runtime Validation Checker")
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
        .get_matches();

    let entry_path = Path::new(matches.get_one::<String>("path").unwrap());

    if !entry_path.exists() {
        eprintln!("Error: path does not exist: {}", entry_path.display());
        std::process::exit(1);
    }

    let rules = match matches.get_one::<String>("rules").map(|s| s.as_str()) {
        Some("zod-lenient") => ValidationRuleSet::ZodLenient,
        Some("custom") => ValidationRuleSet::Custom,
        _ => ValidationRuleSet::ZodStrict,
    };

    let start_time = Instant::now();
    let parsed_modules = parse_dir(entry_path);
    println!("Parsing completed in: {:?}", start_time.elapsed());

    let start_time = Instant::now();
    let violations = analyze_modules(parsed_modules, rules);
    println!("Analysis completed in: {:?}", start_time.elapsed());

    for v in &violations {
        println!("❗ [{}:{}:{}] {}", v.file, v.line, v.column, v.message);
    }

    if matches.get_flag("fail-on-warning") && !violations.is_empty() {
        std::process::exit(1);
    }
}
