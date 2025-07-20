# envy-safe

> A secure, developer-friendly CLI to manage and validate your `.env` files in Rust projects.

![GitHub stars](https://img.shields.io/github/stars/your-user/envy-safe?style=social)

## ✨ Features

- ✅ Validate `.env` against `.env.example`
- 🔄 Sync missing variables from `.env.example`
- 🔐 (Coming Soon) Encrypt/decrypt secrets with `age`
- 🧪 GitHub Action for CI validation
- 📦 Built with Rust, installable via `cargo`

---

## 🚀 Installation

```bash
cargo install envy-safe
```

Or clone it and build:

```bash
git clone https://github.com/your-user/envy-safe.git
cd envy-safe
cargo build --release
```

---

## 🛠 Usage

### Check your `.env` file

```bash
envy-safe --check
```
Will compare `.env` and `.env.example` and report missing keys.

### Sync missing variables
```bash
envy-safe --sync
```
Adds missing keys from `.env.example` to `.env` with example values.

---

## 📋 Example
Given a `.env.example` like:
```
DB_HOST=localhost
DB_PORT=5432
API_KEY=your-api-key
```

If your `.env` is missing `API_KEY`, running `--check` will show:
```
Missing key: API_KEY
```
And running `--sync` will append the missing key to `.env`.

---

## 🔐 Coming Soon
- `envy-safe encrypt`: Encrypt sensitive variables in `.env`
- `envy-safe decrypt`: Decrypt them for runtime use
- GitHub Action: Fail CI if `.env` is out-of-sync
- `cargo envy-safe`: Use it as a Cargo subcommand

---

## 🤝 Contributing
PRs are welcome! Open an issue for feature requests or bugs.

## 📄 License
MIT

---

## ⭐️ Star This Project
If you find this project useful, please consider giving it a star 🌟

---

Made with 💙 by Sergio
