use analysis::analyze_modules;
use clap::{Arg, ArgAction, Command};
use parser::parser::parse_dir;
use std::path::Path;
use std::time::Instant;
use validation::ValidationRuleSet;

fn main() {
    let matches = Command::new("ts-validator")
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
    
    // Display starting analysis message
    let rule_name = match matches.get_one::<String>("rules") {
        Some(name) => name,
        None => "zod-strict",
    };
    println!("🔍 Starting analysis on {} with rules: {}", entry_path.display(), rule_name);

    let start_time = Instant::now();
    println!("\nParsing files:");

    let parsed_modules = parse_dir(entry_path);
    let parsed_modules_len = parsed_modules.len(); 
    println!("\n🛠  Parsing completed in: {:?}", start_time.elapsed());

    println!("\nAnalyzing files:");
    let violations = analyze_modules(parsed_modules, rules);
    let violations_len = violations.len();

    if violations_len > 0 {
        println!("\n❗ violations detected");
    } else {
        println!("\n✅ No violations found");
    }

    for v in &violations {
        println!("\x1b[91m[{}:{}:{}] {}\x1b[0m", v.file, v.line, v.column, v.message);
    }
    println!(
        "\n🏁 Analysis completed in {:?}",
        start_time.elapsed()
    );
    println!(
        "✅ {} files parsed | ❗ {} violations found | 🕑 {:?} total",
        parsed_modules_len,
        violations.len(),
        start_time.elapsed()
    );
    if matches.get_flag("fail-on-warning") && !violations.is_empty() {
        std::process::exit(1);
    }
}
