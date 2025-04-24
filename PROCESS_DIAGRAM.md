# ts-validator -to-End Flow

```text
User CLI
$ ts-validator ./src --rules zod-strict --fail-on-warning
    │
    ▼
┌──────────────────────────────┐
│        CLI Crate (`cli`)     │
│ - Parses command-line args   │
│ - Sets ValidationRuleSet     │
│ - Orchestrates the pipeline  │
│ - Handles --fail-on-warning  │
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│     Parser Crate (`parser`)  │
│ - Finds .ts/.tsx files       │
│ - Resolves relative imports  │
│ - Parses files to SWC ASTs   │
│ - Returns Vec<ParsedModule>  │
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│   Analysis Crate (`analysis`)│
│ - Walks ASTs with visitors   │
│ - Identifies route handlers  │
│ - Tracks aliases (`req.body`)│
│ - Records unvalidated access │
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│ Validation Crate (`validation`) │
│ - Applies rule set logic     │
│ - Matches .parse(), etc.     │
│ - Supports ZodStrict, Lenient│
└────────────┬─────────────────┘
             │
             ▼
┌──────────────────────────────┐
│ Reporting Crate (`reporting`)│
│ - Constructs Violation structs│
│ - Formats output (CLI/JSON)  │
│ - Used by analysis and CLI   │
└────────────┬─────────────────┘
             │
             ▼
🎯 Final Output
- Prints clear violation list
- Supports JSON output (TBD)
- Exit code 1 if `--fail-on-warning`
```