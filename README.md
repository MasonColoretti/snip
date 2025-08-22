---

# Snip – Ultra-Fast CLI Snippet Manager

![snip logo](https://img.shields.io/badge/snip-CLI-blue?style=for-the-badge)

Snip is a **lightweight, blazing-fast, terminal-first snippet manager** for developers, sysadmins, and anyone who works in the command line. Store, search, retrieve, and manage your code or text snippets **without leaving the terminal**. No editors, no fuss, just pure CLI workflow.

---

## 🚀 Features

* **Save snippets instantly**

  ```bash
  snip save hello 'println!("Hello, world!");'
  ```

* **Get snippets directly in your terminal**

  ```bash
  snip get hello
  ```

  Output appears immediately — ready to copy, pipe, or execute. No editor opens, no clipboard required.

* **List all snippets**

  ```bash
  snip list
  ```

  Shows all saved snippets with their IDs and names.

* **Search snippets by name or content**

  ```bash
  snip search println
  ```

  Finds snippets with partial matches and shows a preview for quick selection.

* **Remove snippets safely**

  ```bash
  snip rm hello
  ```

* **Portable and fast**
  Written in **Rust**, runs on Linux, macOS, and Windows. Minimal dependencies, blazing performance.

* **Pure CLI workflow**
  Perfect for piping directly into commands:

  ```bash
  snip get hello | bash
  ```

---

## ⚡ Installation

### From source

```bash
git clone https://github.com/yourusername/snip.git
cd snip
cargo build --release
cp target/release/snip /usr/local/bin/
```

### Dependencies

* Rust 1.79+
* SQLite3 (bundled with rusqlite)
* Works headless — no X server or clipboard needed

---

## 💡 Example Usage

```bash
# Save a snippet
snip save greet 'echo "Hello, world!"'

# Retrieve it
snip get greet

# List all snippets
snip list

# Search snippets containing 'echo'
snip search echo

# Remove a snippet
snip rm greet
```

---

## 🔧 Why Snip?

Snip is **designed for developers who live in the terminal**. Unlike bloated snippet managers:

* No GUI required
* Works in headless environments
* Fast, lightweight, Rust-powered
* Searchable, printable, ready for piping or redirecting

It’s like **your shell history on steroids**, but smarter.

---

## 📂 Data Storage

All snippets are stored locally in an **SQLite database** in a system-standard directory:

* **Linux**: `~/.local/share/snip/snip.db`
* **macOS**: `~/Library/Application Support/snip/snip.db`
* **Windows**: `%APPDATA%\snip\snip.db`

---

## ⚙️ Contribution

Snip is open source! Pull requests, bug reports, or feature ideas are welcome. Help make Snip **the ultimate CLI snippet tool**.

---

## 🌟 Star Features

* Pure CLI, zero editor popups
* Searchable, pipeable, and script-friendly
* Written in Rust for **maximum performance**
* Works anywhere — headless servers, WSL, remote SSH

---

**Snip — your snippets, at the speed of thought.**

---

