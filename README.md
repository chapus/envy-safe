# envy-safe

> A secure, developer-friendly CLI to manage and validate your `.env` files in Rust projects.

![GitHub stars](https://img.shields.io/github/stars/chapus/envy-safe?style=social)

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

### 📦 As a Cargo Subcommand

Once installed, you can use `envy-safe` like any built-in cargo command:

```bash
cargo envy-safe --check
cargo envy-safe --sync
cargo envy-safe --encrypt API_KEY
cargo envy-safe --decrypt API_KEY

---

### 🔐 Encrypting Values

To encrypt values in your `.env` file, set your [age](https://github.com/FiloSottile/age) public key as an environment variable:

```bash
export ENVY_AGE_RECIPIENT="age1xyz..."
```

Then run:

```bash
envy-safe --encrypt API_KEY
```

You can also create a config file at:

- Linux/macOS: `~/.config/envy-safe/config.toml`
- Windows: `%APPDATA%\envy-safe\config.toml`

```toml
recipient = "age1xyz..."
```

> If `ENVY_AGE_RECIPIENT` is not set and no config file is found, encryption will fail with an informative message.

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
