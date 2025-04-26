# CONTRIBUTING to ts-validator

Thanks for your interest in improving **ts-validator**!  
Here’s how to get set up for local development.

---

## 🛠️ Local Setup

1. **Clone the repository:**

```bash
git clone git@github.com:Stephen-Collins-tech/ts-validator.git
cd ts-validator
```

2. **Build the project:**

```bash
cargo build
```

This will compile the tool into `target/debug/ts-validator`.

3. **Run against a TypeScript file or directory:**

```bash
cargo run -- path/to/entry-file.ts
```

Or, after building:

```bash
./target/debug/ts-validator path/to/entry-file.ts
```

---

## 🧩 Project Structure

- **`crates/cli/`** — Command-line entry point (argument parsing, flags).
- **`crates/analysis/`** — Core analysis logic (detection engine, AST traversal).
- **`crates/parser/`** — TypeScript file parsing, import resolution.
- **`crates/reporting/`** — Output formats, violation reporting.
- **`crates/utils/`** - Utility functions for the project.
- **`crates/validation/`** - Validation logic for the project.

Each crate is organized as part of a Cargo workspace.

---

## 🚀 Common Commands

| Task                      | Command                          |
|----------------------------|----------------------------------|
| Build (debug)              | `cargo build`                   |
| Build (release)            | `cargo build --release`         |
| Run CLI                   | `cargo run -- path/to/file.ts`  |
| Format code                | `cargo fmt`                     |
| Check for warnings/errors  | `cargo check`                   |
| Run tests (once added)     | `cargo test`                    |

---

## 🧹 Code Style

- Run `cargo fmt` before submitting any pull requests.
- Favor clear, minimal code over clever tricks.
- Comments welcome, especially for tricky AST logic!

---

## 📈 Development Roadmap

MVP goals are tracked in [`TASKS.md`](./TASKS.md).

If you're looking for a good place to start contributing, check the issues labeled **`good first issue`**.

---

## 📦 Code Coverage

To generate a code coverage report, first install the `cargo-llvm-cov` tool:

```bash
cargo install cargo-llvm-cov
```

Then, run the following command to generate a code coverage report:

```bash
cargo llvm-cov --html
```

This will generate a code coverage report in the `target/llvm-cov/html` directory.

---

# 🤝 Pull Request Guidelines

- Small, focused PRs are easier to review.
- Include clear commit messages.
- If adding a feature, briefly explain the motivation in the PR description.

---

Thanks again for helping make **ts-validator** better! 🎯