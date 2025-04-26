use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn get_test_app_routes_path() -> PathBuf {
    // Get the path to user routes file which has validation issues
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up from crates/cli
    path.pop(); // Go up from crates
    path.push("examples");
    path.push("test-express-app");
    path.push("src");
    path.push("routes");
    path.push("userRoutes.ts");
    path
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("ts-validator").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: ts-validator"));
}

#[test]
fn test_express_app_zod_strict() {
    let mut cmd = Command::cargo_bin("ts-validator").unwrap();
    let file_path = get_test_app_routes_path();
    cmd.arg(file_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("🔍 Starting analysis on"))  // Check for starting message
        .stdout(predicate::str::contains("with rules: zod-strict"))   // Check for rule name
        .stdout(predicate::str::contains("Analysis completed"))
        .stdout(predicate::str::contains("controllers found"))        // Check for controller summary
        .stdout(predicate::str::contains("Unvalidated"));             // Specific error message
}

#[test]
fn test_express_app_zod_lenient() {
    let mut cmd = Command::cargo_bin("ts-validator").unwrap();
    let file_path = get_test_app_routes_path();
    cmd.arg("--rules")
       .arg("zod-lenient")
       .arg(file_path);
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("🔍 Starting analysis on"))   // Check for starting message
       .stdout(predicate::str::contains("with rules: zod-lenient"))   // Check for rule name
       .stdout(predicate::str::contains("Analysis completed"))
       .stdout(predicate::str::contains("controllers found"));        // Check for controller summary
}

#[test]
fn test_express_app_custom_rules() {
    let mut cmd = Command::cargo_bin("ts-validator").unwrap();
    let file_path = get_test_app_routes_path();
    cmd.arg("--rules")
       .arg("custom")
       .arg(file_path);
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("🔍 Starting analysis on"))   // Check for starting message
       .stdout(predicate::str::contains("with rules: custom"))        // Check for rule name
       .stdout(predicate::str::contains("Analysis completed"))
       .stdout(predicate::str::contains("controllers found"));        // Check for controller summary
}

#[test]
fn test_express_app_fail_on_warning() {
    let mut cmd = Command::cargo_bin("ts-validator").unwrap();
    let file_path = get_test_app_routes_path();
    cmd.arg("--fail-on-warning")
       .arg(file_path);
    cmd.assert()
        .failure() // Should fail because of the violations 
        .stdout(predicate::str::contains("🔍 Starting analysis on"))  // Check for starting message
        .stdout(predicate::str::contains("with rules: zod-strict"))   // Check for rule name
        .stdout(predicate::str::contains("Analysis completed"))
        .stdout(predicate::str::contains("controllers found"));       // Check for controller summary
}

// Add more tests here... 