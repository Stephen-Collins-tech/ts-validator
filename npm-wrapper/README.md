# ts-validator

[![npm version](https://img.shields.io/npm/v/@stephen-collins-tech/ts-validator.svg)](https://www.npmjs.com/package/@stephen-collins-tech/ts-validator)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Rust-powered static analysis for TypeScript APIs.**  
Find unvalidated `req.body`, `req.query`, and `req.params` — before they cause runtime errors or security vulnerabilities.

## 🚀 Quickstart

No install needed. Just run:

```bash
npx @stephen-collins-tech/ts-validator path/to/entry-file.ts
```

Options:

```bash
npx @stephen-collins-tech/ts-validator --help
```

## ✨ What It Does

- Recursively parses `.ts` and `.tsx` files
- Detects raw access to external inputs like `req.body`
- Flags usage that may lack proper runtime validation
- Supports basic flags: `--json`, `--fail-on-warning`, `--help`, `--version`

## 🛡️ Why ts-validator?

TypeScript protects you at **compile time**.  
**ts-validator** protects you at **runtime** — when external data actually hits your app.

Don't trust unvalidated input. Catch it automatically.

## 📦 Installation

### Global Installation

```bash
npm install -g @stephen-collins-tech/ts-validator
ts-validator --version
```

### Project Installation

```bash
npm install --save-dev @stephen-collins-tech/ts-validator
```

Then in your package.json:
```json
"scripts": {
  "validate": "ts-validator src/index.ts"
}
```

## License

MIT License. 