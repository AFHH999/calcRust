
# Rust CLI Calculator & Historian

A modular, test-driven command-line calculator built with Rust. This project prioritizes **separation of concerns**, **error resilience**, and **auditable persistence**.

## 🏗️ Architectural Design

Unlike basic scripts, this application is built as a **Library-First** system. The core logic is decoupled from the user interface and the file system, ensuring the "Engine
portable and 100% testable.

### 1. Decoupled Logic (`lib.rs`)

The library handles the "Pure" operations:

* **Mathematical Evaluation**: Safe floating-point arithmetic with explicit zero-division checks.
* **Serialization**: Leveraging `serde` to transform internal states into structured JSON.
* **I/O Abstraction**: Functions are designed with **Inversion of Control**, accepting paths as arguments to prevent side effects during testing.

### 2. Interface Layer (`main.rs`)

The binary acts as a controller:

* **Input Validation**: Strict type-checking loops for integers, floats, and operators.
* **Error Mapping**: Uses `.map_err(std::io::Error::other)?` to translate foreign error types into native I/O results, preventing unexpected crashes (DoS) and ensuring a
"fail-safe" execution path.

## 🛡️ Security & Reliability Features

* **JSON Integrity**: History is stored in structured JSON format, making the logs auditable and resistant to simple "string-injection" corruption.
* **Panic-Free Design**: Replaces `unwrap()` with robust `match` and `?` operators to handle missing files or corrupted data without terminating the process.
* **Atomic File Operations**: Uses `OpenOptions` with append/create flags to ensure data persistence follows a predictable, non-destructive path.

## 🚀 Usage

### Requirements

* **Rust** (2024 Edition)
* **Arch Linux** (or any Unix-like system)
* **Tarpaulin** (Optional, for coverage reports)
* **rusqlite** (Necessary for the data base)

### Execution

Build and run the interactive CLI:
o run

## Testing & Validation

The project uses a comprehensive suite of unit tests to verify mathematical accuracy and formatting logic.
cargo test

cargo-tarpaulin generate a code coverage report (requires `cargo-tarpaulin`):

# 🛠️ Tech Stack

    **Language**: Rust
    **Indentation**: Tabs (for readability and consistency)
