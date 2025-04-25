# Project Architecture

This document outlines the structure and purpose of the crates within the ts-validator project, organized as a Cargo workspace.

## Workspace Structure

- The root `Cargo.toml` defines the workspace members.
- Each crate lives under the `crates/` directory and has its own `Cargo.toml`.

## Crates

### Root `Cargo.toml`
- **Purpose:** Declares all workspace member crates and applies shared settings.
- **Responsibilities:**
    - Defines workspace members.
    - Optionally configures shared dependencies and profiles.

---

### `crates/cli` – **Command-Line Interface**
- **Type:** Binary crate
- **Purpose:** Top-level entry point for running the tool from the terminal.
- **Responsibilities:**
    - Parses command-line arguments using `clap`.
    - Accepts options like `--fail-on-warning` and `--rules`.
    - Initializes the `ValidationRuleSet` from user input.
    - Orchestrates parsing and analysis.
    - Outputs violations in CLI format.
    - Sets appropriate exit code based on violations and flags.

---

### `crates/parser` – **File Discovery and AST Parsing**
- **Type:** Library crate
- **Purpose:** Finds TypeScript source files and parses them into SWC ASTs.
- **Responsibilities:**
    - Recursively discovers `.ts` and `.tsx` files.
    - Parses files using `swc_ecma_parser`.
    - Associates ASTs with file metadata and a `SourceMap`.
    - Exposes a `ParsedModule` struct.

---

### `crates/analysis` – **AST Traversal and Violation Detection**
- **Type:** Library crate
- **Purpose:** Runs analysis visitors on each parsed AST.
- **Responsibilities:**
    - Walks AST nodes to identify unvalidated input usage.
    - Detects access to `req.body`, `req.params`, `req.query`, and aliasing patterns.
    - Tracks whether any `z.parse()`-style validations were applied.
    - Delegates validation checks to the `validation` crate.
    - Emits `Violation` records with location and message.

---

### `crates/validation` – **Validation Rule Engine**
- **Type:** Library crate
- **Purpose:** Centralizes logic for detecting whether a piece of code performs input validation.
- **Responsibilities:**
    - Defines the `ValidationRuleSet` enum.
    - Implements detection functions like `is_validation_call()`.
    - Supports configurable rule modes (e.g., `ZodStrict`, `ZodLenient`, `Custom`).
    - Allows extension with future rule sets.

---

### `crates/reporting` – **Violation Structs and Output Formatting**
- **Type:** Library crate
- **Purpose:** Defines how violations are stored and output.
- **Responsibilities:**
    - Defines `Violation` and `ViolationKind` structs.
    - Tracks metadata (file, line, column, message, type).
    - Future: supports JSON and other formats.
    - Used by both `analysis` and `cli`.

---

### `crates/utils`
- **Type:** Library crate 
- **Purpose:** Contains reusable formatting helpers (e.g., for location strings).
- **Responsibilities:**
    - Formats `[file:line:col]` strings.
    - Avoids code duplication across analysis/reporting.