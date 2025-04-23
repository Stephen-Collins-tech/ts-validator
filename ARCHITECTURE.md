# Project Architecture

This document outlines the structure and purpose of the crates within the TypeScript Runtime Validation Checker project, organized as a Cargo workspace.

## Workspace Structure

- The root `Cargo.toml` defines the workspace members.
- Individual crates reside within the `crates/` directory.

## Crates

### Root `Cargo.toml`
- **Purpose:** Defines the Cargo workspace and points to the member crates located in `crates/`.
- **Responsibilities:**
    - Defines workspace members.
    - Can define shared workspace-level dependencies or build profiles.

### `TASKS.md`
- **Purpose:** Tracks the Minimum Viable Product (MVP) requirements and potential future tasks.
- **Responsibilities:**
    - Lists features and functionalities to be implemented.
    - Acts as a checklist for development progress.
    - Stays at the root level.

### `crates/cli` (Binary Crate)
- **Purpose:** The main executable entry point for the CLI tool (`ts-validator` or similar). Contains `src/main.rs`.
- **Responsibilities:**
    - Parses command-line arguments using `clap`.
    - Depends on the other workspace crates (`error`, `parser`, `analysis`, `reporting`).
    - Instantiates and orchestrates the components from the library crates.
    - Handles top-level error reporting to the user (using `eprintln!`).
    - Determines the final exit code of the application based on success, errors, or the `--fail-on-warning` flag.

### `crates/error` (Library Crate)
- **Purpose:** Defines the shared custom error types for the application. Contains `src/lib.rs`.
- **Responsibilities:**
    - Contains the main `AnalyzerError` enum (using `thiserror`).
    - Provides specific variants for different error conditions (I/O, parsing, resolution, analysis) to allow for granular error handling across other crates.

### `crates/parser` (Library Crate)
- **Purpose:** Handles file discovery, reading, parsing, and import resolution. Contains `src/lib.rs`.
- **Responsibilities:**
    - Depends on `error`.
    - Finding all `.ts` files within a given directory (`walkdir`).
    - Managing the SWC `SourceMap`.
    - Parsing TypeScript source code into Abstract Syntax Trees (ASTs) using `swc_ecma_parser`.
    - Caching parsed ASTs to avoid redundant work and handle circular dependencies.
    - Resolving module specifiers (e.g., `"../utils/validation"`) to absolute file paths.

### `crates/analysis` (Library Crate)
- **Purpose:** Contains the core analysis logic for detecting unvalidated input usage. Contains `src/lib.rs`.
- **Responsibilities:**
    - Depends on `error`, `parser`, and `heuristics`.
    - Implements the `swc_ecma_visit::Visit` trait to traverse the ASTs.
    - Identifies potential access points for request inputs (`req.body`, `req.params`, `req.query`), including aliased variables.
    - Uses the `parser` crate to request parsed ASTs (including resolving imports).
    - Uses the `heuristics` crate to determine if an identified input usage is considered "validated".
    - Generates `Violation` data structures (defined potentially in `reporting` or a shared types crate).

### `crates/heuristics` (Library Crate)
- **Purpose:** Defines the rules and logic for determining if a request input usage is considered validated. Contains `src/lib.rs`.
- **Responsibilities:**
    - Depends on `error`.
    - Contains functions or data structures that represent validation patterns (e.g., specific function calls like `z.object(...).parse()`, wrappers like `validate(...)`).
    - Takes AST nodes (or relevant context) as input and returns whether the usage meets the defined validation criteria.

### `crates/reporting` (Library Crate)
- **Purpose:** Handles the definition of violation types and the formatting/output of analysis results. Contains `src/lib.rs`.
- **Responsibilities:**
    - Depends on `error`.
    - Defines the `Violation` struct (containing file path, line/column, message, etc.).
    - Formats a list of `Violation`s into a human-readable string for console output.
    - Formats a list of `Violation`s into a JSON string for machine-readable output (`--json` flag). 