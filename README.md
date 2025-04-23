# TypeScript Runtime Validation Checker

This project is a Rust-based static analysis tool designed to enhance runtime type safety in TypeScript applications, particularly those handling external requests (e.g., web servers). It parses TypeScript codebases to detect potentially unvalidated usage of request inputs like `req.body`, `req.params`, and `req.query`.

The primary goal is to prevent runtime errors and security vulnerabilities that can arise from trusting external data without proper validation.

## Current Features (MVP In Progress)

*   **TypeScript Parsing:** Recursively parses `.ts` and `.tsx` files starting from a specified entry point (file or directory).
*   **AST Generation:** Uses the SWC library to generate Abstract Syntax Trees (ASTs) for each parsed file.
*   **Relative Import Resolution:** Follows relative import paths (`import x from './module'`) to analyze the entire relevant codebase.
*   **CLI Interface:** Provides a basic command-line interface using `clap`:
    *   Specify entry path (file or directory) via positional argument.
    *   `--json` flag (reporting mechanism TBD).
    *   `--fail-on-warning` flag (CI/CD feature TBD).
    *   Includes `--help` and `--version`.
*   **Basic Detection:** Includes an initial AST visitor pass that identifies *all* member access expressions matching `req.body`, `req.params`, or `req.query`. (Note: This currently flags all accesses, validation context checks are pending).

## How to Build & Run (Example)

1.  **Build:**
    ```bash
    cargo build
    ```
2.  **Run (e.g., on the test application):**
    ```bash
    # Using cargo run
    cargo run -- test-app/src/index.ts
    # Or using the compiled binary
    ./target/debug/rust-typescript-ast-parsing test-app/src/index.ts
    ```
    *(Replace `rust-typescript-ast-parsing` with your actual package name if different)*

## MVP Roadmap (Tasks from `TASKS.md`)

1.  **CLI Enhancements:** (Partially Done)
    *   Implement functionality for `--json` output.
    *   Implement `--fail-on-warning` exit code behavior.
2.  **Detection Pass Refinements:**
    *   Handle `req` object aliasing (e.g., `const { body } = req; console.log(body);`).
    *   Implement validation heuristics to distinguish validated vs. unvalidated access (e.g., checking for calls like `zodSchema.parse(req.body)` nearby).
3.  **Reporting:**
    *   Define a `Violation` struct with file, line, column, kind, and expression details.
    *   Implement human-readable CLI output format.
    *   Implement JSON output format when `--json` is used.
4.  **CI/CD:**
    *   Ensure the tool exits with a non-zero status code if violations are found and `--fail-on-warning` is set.

## Future Goals (Out of Scope for MVP)

*   VSCode extension
*   GitHub bot integration
*   Rule configuration (`.ts-validatorrc`)
*   Support for decorator-based validation (e.g., `class-validator`)
