#!/bin/bash

# Script to generate and optionally open code coverage reports using cargo llvm-cov

set -e # Exit immediately if a command exits with a non-zero status.

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
WORKSPACE_ROOT=$(realpath "$SCRIPT_DIR/..")
# Output directory for llvm-cov reports
REPORT_DIR="$WORKSPACE_ROOT/target/llvm-cov"
# Default HTML report entry point
HTML_REPORT="$REPORT_DIR/html/index.html"
# Directory for raw profile data
PROFRAW_DIR="$WORKSPACE_ROOT/target/llvm-profraw"

# Base arguments for cargo llvm-cov
LLVM_COV_ARGS=(
    "llvm-cov" # Subcommand for cargo
    "--all"    # Run for all crates in the workspace (or use --workspace)
    "--html"   # Generate HTML report
    "--output-dir" "$REPORT_DIR"
    # Add other llvm-cov options if needed, e.g., --ignore-filename-regex
)

OPEN_REPORT=false

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --open)
            OPEN_REPORT=true
            shift # past argument
            ;;
        # Add other arguments here if needed
        # e.g., --lcov --output-path lcov.info for LCOV format
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--open]"
            exit 1
            ;;
    esac
done

# Ensure cargo llvm-cov is available (requires llvm-tools-preview component)
if ! cargo llvm-cov --version &> /dev/null; then
    echo "Error: cargo llvm-cov command failed."
    echo "Ensure you have the llvm-tools-preview component installed:"
    echo "  rustup component add llvm-tools-preview"
    echo "And potentially update cargo-llvm-cov itself:"
    echo "  cargo install cargo-llvm-cov --locked"
    exit 1
fi

# Go to workspace root
cd "$WORKSPACE_ROOT"

echo "Running cargo llvm-cov..."
echo "Report will be generated in: $REPORT_DIR/html"

# Create/clear profile data directory
echo "Ensuring profile data directory exists: $PROFRAW_DIR"
mkdir -p "$PROFRAW_DIR"

# Clear previous report if it exists, as llvm-cov might not overwrite cleanly
if [ -d "$REPORT_DIR" ]; then
  echo "Removing previous report directory: $REPORT_DIR"
  rm -rf "$REPORT_DIR"
fi

# Set necessary environment variable for llvm-cov
export RUSTFLAGS="-Cinstrument-coverage"
# This profile ensures coverage even for dependencies and avoids stripping symbols
# Place raw profile files in the dedicated directory
export LLVM_PROFILE_FILE="$PROFRAW_DIR/coverage-%p-%m.profraw"

# Run tests first to generate profile data
echo "Running tests to generate coverage data..."
cargo test --all --no-fail-fast

# Run llvm-cov to generate the report from profile data
echo "Generating coverage report..."
cargo "${LLVM_COV_ARGS[@]}"

# Unset the environment variables
unset RUSTFLAGS
unset LLVM_PROFILE_FILE

# Clean up raw profile data files if they exist in the designated directory
find "$PROFRAW_DIR" -maxdepth 1 -name 'coverage-*.profraw' -delete

echo "Coverage report generated: $HTML_REPORT"

# Open the report if requested
if [ "$OPEN_REPORT" = true ]; then
    echo "Opening report in default browser..."
    # Use appropriate command for the OS
    case "$(uname -s)" in
        Linux*)
            xdg-open "$HTML_REPORT" &> /dev/null || echo "Could not open report automatically."
            ;;
        Darwin*)
            open "$HTML_REPORT" || echo "Could not open report automatically."
            ;;
        CYGWIN*|MINGW*|MSYS*|Windows*) # Added Windows_NT for typical cmd/powershell uname
            start "$HTML_REPORT" || echo "Could not open report automatically."
            ;;
        *)
            echo "Unsupported OS for automatic opening. Please open the report manually:"
            echo "$HTML_REPORT"
            ;;
    esac
fi

echo "Done." 