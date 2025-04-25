# ts-validator

**Rust-powered static analysis for TypeScript APIs.**  
Find unvalidated `req.body`, `req.query`, and `req.params` — before they cause runtime errors or security vulnerabilities.

---

## 🚀 Quickstart

No install needed. Just run:

```bash
npx ts-validator path/to/entry-file.ts
```

Options:

```bash
npx ts-validator --help
```

---

## ✨ What It Does

- Recursively parses `.ts` and `.tsx` files.
- Detects raw access to external inputs like `req.body`.
- Flags usage that may lack proper runtime validation.
- Supports basic flags: `--json`, `--fail-on-warning`, `--help`, `--version`.

---

## 🛡️ Why ts-validator?

TypeScript protects you at **compile time**.  
**ts-validator** protects you at **runtime** — when external data actually hits your app.

Don't trust unvalidated input. Catch it automatically.

---

## 🧰 For Contributors

Want to build locally or extend the tool?  
See [CONTRIBUTING.md](./CONTRIBUTING.md).

---

# Example Usage

```bash
npx ts-validator src/index.ts
```

Output:

```
Found 2 potential unvalidated accesses:
  - src/routes/user.ts:14 -> req.body
  - src/routes/login.ts:22 -> req.query
```

---

# 📈 Roadmap

- Smarter validation detection (`schema.parse(req.body)`)
- JSON reporting mode (`--json`)
- CI/CD support (`--fail-on-warning`)

---

# License

MIT License.