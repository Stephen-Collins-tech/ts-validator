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

#[test]
fn test_express_app_json_output() {
    let mut cmd = Command::cargo_bin("ts-validator").unwrap();
    let file_path = get_test_app_routes_path();
    cmd.arg("--json")
       .arg(file_path);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");

    // Attempt to parse the output as JSON
    let json_value: Result<serde_json::Value, _> = serde_json::from_str(&output_str);
    assert!(json_value.is_ok(), "Output should be valid JSON");

    let json_data = json_value.unwrap();

    // Check for the presence of expected top-level keys
    assert!(json_data.get("files_parsed").is_some(), "JSON should contain 'files_parsed'");
    assert!(json_data.get("violations_count").is_some(), "JSON should contain 'violations_count'");
    assert!(json_data.get("elapsed_time_ms").is_some(), "JSON should contain 'elapsed_time_ms'");
    assert!(json_data.get("violations").is_some(), "JSON should contain 'violations'");

    // Since the default rules (zod-strict) find violations in the test app,
    // check if the violations array is not empty.
    let violations = json_data["violations"].as_array().expect("Violations should be an array");
    assert!(!violations.is_empty(), "Violations array should not be empty for zod-strict rules");
}

// Add more tests here... 
#[test]
fn test_express_app_json_output_and_fail_on_warning() {
    let mut cmd = Command::cargo_bin("ts-validator").unwrap();
    let file_path = get_test_app_routes_path();
    cmd.arg("--json")
       .arg("--fail-on-warning")
       .arg(file_path);

    // First, assert that the command fails as expected
    let assert = cmd.assert().failure().code(1);

    // Then, get the output from the assertion result
    let output = assert.get_output().stdout.clone();
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");

    // Attempt to parse the output as JSON
    let json_value: Result<serde_json::Value, _> = serde_json::from_str(&output_str);
    assert!(json_value.is_ok(), "Output should be valid JSON even on failure");

    let json_data = json_value.unwrap();

    // Check for the presence of expected top-level keys
    assert!(json_data.get("files_parsed").is_some(), "JSON should contain 'files_parsed'");
    assert!(json_data.get("violations_count").is_some(), "JSON should contain 'violations_count'");
    assert!(json_data.get("elapsed_time_ms").is_some(), "JSON should contain 'elapsed_time_ms'");
    assert!(json_data.get("violations").is_some(), "JSON should contain 'violations'");

    // Since the default rules (zod-strict) find violations in the test app,
    // check if the violations array is not empty.
    let violations = json_data["violations"].as_array().expect("Violations should be an array");
    assert!(!violations.is_empty(), "Violations array should not be empty for zod-strict rules");
}


// Add more tests here... 
