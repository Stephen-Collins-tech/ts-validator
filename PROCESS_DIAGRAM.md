# TypeScript Runtime Validation Checker – End-to-End Flow

```text
User CLI
$ ts-validator ./src --json --fail-on-warning
    │
    ▼
┌──────────────────────────────┐
│        CLI Crate (`cli`)     │
│ - Parses command-line args   │
│ - Orchestrates flow          │
│ - Handles exit code logic    │
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│     Parser Crate (`parser`)  │
│ - Recursively finds .ts files│
│ - Resolves imports           │
│ - Parses into SWC ASTs       │
│ - Handles circular refs      │
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│   Analysis Crate (`analysis`)│
│ - Traverses ASTs             │
│ - Finds req.body/params/etc  │
│ - Tracks aliasing            │
│ - Asks heuristics if safe →  │
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│ Heuristics Crate (`heuristics`) │
│ - Validates input usage      │
│ - e.g. z.object().parse(...) │
│ - Flags unsafe access        │
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│ Reporting Crate (`reporting`)│
│ - Builds Violation structs   │
│ - Outputs CLI/JSON reports   │
└────────────┬─────────────────┘
             │
             ▼
Final Output
- Human-readable output
- Or JSON if `--json`
- Exits with 0 or 1 based on `--fail-on-warning`
