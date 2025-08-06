# 🦀 RustLog

A blazing-fast, modular CLI tool for filtering log files — written in Rust.  
Built to scale from beginner CLI utilities to advanced log streaming with Kafka, plugins, and optional dashboards.

---

## ✨ Features

- 🧪 Filter logs by keyword (e.g., "ERROR", "WARN")
- 📂 Read `.log` files line-by-line
- ⚡ Fast and memory-efficient
- 🧱 Modular, well-structured Rust code
- 🧰 Built with `clap`, `anyhow`, and the Rust standard library

---

## 🚀 Getting Started

### 📦 Prerequisites
- [Install Rust](https://www.rust-lang.org/tools/install)

### 🧑‍💻 Clone & Run

```bash
git clone https://github.com/HarshSharma009/RustLog.git
cd rustlog
cargo run -- ./sample/sample.log ERROR
```
### 🔧 Sample Output

```text
ERROR: Failed to connect
ERROR: Timeout
```

### 🔬 Running Tests

```bash
cargo test
```

---

📁 Project Structure
```python
rustlog/
├── src/
│   ├── main.rs       # CLI entrypoint
│   ├── args.rs       # CLI arguments with `clap`
│   ├── reader.rs     # File reading logic
│   ├── filter.rs     # Keyword filtering
├── tests/
│   ├── filter_tests.rs
│   └── cli_tests.rs
├── Cargo.toml
└── README.md
```
---

## 📦 Tech Stack
- Rust
- clap for CLI parsing
- anyhow for error handling

---
## Roadmap

| Feature                          | Status       |
| -------------------------------- | ------------ |
| Basic CLI with filter            | ✅ Done       |
| Real-time tailing (`tail -f`)    | 🔜 Coming Up |
| Kafka output support             | 🔜           |
| Configurable filters via `.toml` | 🔜           |
| Plugin system for transforms     | 🔜           |
| Web dashboard (Axum + WebSocket) | Optional 🚧  |
---
### 🧠 Learning Outcomes
This project is part of a full roadmap to learn Rust like a pro:

- Ownership, Borrowing, Lifetimes
- Modular architecture
- Concurrency and async
- Real-world crates and ecosystems
- Extensibility patterns

---
## 🤝 Contributing
Pull requests welcome!
Feel free to fork and submit improvements or extensions.


---
## 📜 License
GNU GENERAL PUBLIC LICENSE 