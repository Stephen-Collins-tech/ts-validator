# 🛠️ TypeScript Runtime Validation Checker – Open Core MVP Tasks

This project is a Rust-based static analysis tool that parses TypeScript files and detects unvalidated usage of request inputs (e.g. `req.body`, `req.params`, `req.query`). The goal is to enforce **runtime type safety** and provide a CI/CD-friendly CLI.

---

## ✅ Base Functionality (Existing)

- [ ] Parse `.ts` files into SWC ASTs
- [ ] Generate SWC AST

---

## 🔨 TODO: MVP Detection Tool

### 1. CLI Interface
- [ ] Add argument parsing using `clap` or similar
  - [ ] `--path` or positional: directory or file
  - [ ] `--json` (output machine-readable report)
  - [ ] `--fail-on-warning` (sets non-zero exit code if violations)
- [ ] Add help/version commands

### 2. Detection Pass: Unvalidated Input Access
- [ ] Recursively find `.ts` files starting from the entry point.
- [ ] Resolve relative imports to build a basic module dependency understanding.
Implement a visitor using `swc_ecma_visit` to detect usage of request input properties.

- [ ] Walk through AST and detect:
  - [ ] `req.body`
  - [ ] `req.params`
  - [ ] `req.query`
- [ ] Detect `req` aliasing (e.g. `const { body } = req`)
- [ ] Flag usage of these values **outside of validation context**

### 3. Validation Heuristics
Define what counts as "validated":

- [ ] `z.object(...).parse(req.body)`
- [ ] `validate(req.body)` or similar
- [ ] `const x = req.body; zodSchema.parse(x);` (track variable flow — optional for MVP)
- [ ] Skip checking decorators (`@IsString`, etc.) for now

### 4. Report Output
- [ ] Create `Violation` struct with:
  - `file`, `line`, `column`, `kind`, `expression`
- [ ] Output violations to:
  - [ ] Human-readable CLI format
  - [ ] JSON if `--json` flag is used

### 5. CI/CD Friendly
- [ ] If any violation is found and `--fail-on-warning` is set:
  - [ ] Print summary
  - [ ] Exit with code 1
- [ ] Otherwise, exit with 0

---

## 🧪 (Optional) Tests
- [ ] Create fixtures for:
  - [ ] Validated usage (should not flag)
  - [ ] Unvalidated usage (should flag)
- [ ] Add integration test that runs CLI on fixture directory

---

## 📦 Future (Out of Scope for MVP)
- [ ] VSCode extension
- [ ] GitHub bot integration
- [ ] Rule configuration support (`.ts-validatorrc`)
- [ ] Cross-repo schema diffing
- [ ] Support for decorators / class-validator