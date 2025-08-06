# 🦀 RustLog

A blazing-fast, modular CLI tool for filtering log files — written in Rust.  
Built to scale from beginner CLI utilities to advanced log streaming with Kafka, plugins, and optional dashboards.

---

## ✨ Features

- 🔍 Filter log files by keyword (e.g. "ERROR")
- 📡 Real-time log tailing (`--tail` mode)
- ✨ Logging via `env_logger`
- 🛑 Graceful shutdown on Ctrl+C
- 🧪 Unit + CLI tests using `assert_cmd`, `tempfile`, etc.
- 💼 Structured for modularity and easy scaling

---

## 🚀 Getting Started

### 📦 Prerequisites
- [Install Rust](https://www.rust-lang.org/tools/install)

### 🧑‍💻 Clone & Run

```bash
git clone https://github.com/HarshSharma009/RustLog.git
cd rustlog
cargo build --release
RUST_LOG=info cargo run -- ./sample/sample.log ERROR
```
### 🔧 Sample Output

```text
ERROR: Failed to connect
ERROR: Timeout
```

### Tailing Mode (--tail)
```bash
RUST_LOG=info cargo run -- ./sample/sample.log ERROR --tail
```
#### Try this in a second terminal:
```bash
echo "ERROR: Out of memory!" >> ./sample/sample.log
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
│   ├── cli_tests.rs
│   └── tail_tests.rs
├── sample/
│   └── sample.log
├── Cargo.toml
└── README.md
```
---

## 🧰 Built With
- clap – for argument parsing
- env_logger – for runtime logging
- anyhow – for easy error handling
- ctrlc – for graceful shutdown
- assert_cmd – for CLI testing
- tempfile – for temporary file-based testing


## Roadmap

| Feature                          | Status       |
| -------------------------------- | ------------ |
| Basic CLI with filter            | ✅ Done      |
| Real-time tailing (`tail -f`)    | ✅ Done      |
| Kafka output support             | 🔜           |
| Configurable filters via `.toml` | 🔜           |
| Plugin system for transforms     | 🔜           |
| Web dashboard (Axum + WebSocket) | Optional 🚧  |


## 📌 TODO (Optional Enhancements)

- [ ] Async tailing with [`tokio`](https://docs.rs/tokio)
- [ ] Output color-coded log levels (e.g., red for `ERROR`, yellow for `WARN`)
- [ ] JSON log format support
- [ ] Export filtered output to a new file (`--out <file>`)

### 🧠 Learning Outcomes
This project is part of a full roadmap to learn Rust like a pro:

- Ownership, Borrowing, Lifetimes
- Modular architecture
- Concurrency and async
- Real-world crates and ecosystems
- Extensibility patterns


## 🤝 Contributing
Pull requests welcome!
Feel free to fork and submit improvements or extensions.



## 👨‍💻 Author
**Harsh Sharma**  
[GitHub](https://github.com/HarshSharma009) | [LinkedIn](https://www.linkedin.com/in/harsh-sharma-8a850b173/)  
📧 harshsharma.ext@gmail.com

> _“Rusting my way through logs 🦀”_


## 📜 License
GNU GENERAL PUBLIC LICENSE 