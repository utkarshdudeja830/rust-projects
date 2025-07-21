# Rust Projects

A collection of Rust programming projects demonstrating fundamental concepts, algorithms, and advanced features like mutexes and async programming. Each project is a standalone Rust crate with its own `Cargo.toml` and source files, inspired by the Rust programming book and extended with custom implementations.

## Project Structure
```
.
├── 1-hello_cargo
│   ├── Cargo.toml
│   └── src
│       └── main.rs           # Basic "Hello, World!" program
├── 2-guessing_game
│   ├── Cargo.toml
│   └── src
│       └── main.rs           # Number guessing game
├── 3-programming_concepts
│   ├── 3.1-temperature_conversion
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs       # Temperature converter
│   └── 3.2-nth_fibbonacci_number
│       ├── Cargo.toml
│       └── src
│           └── main.rs       # Nth Fibonacci number generator
├── 4-collections
│   └── 4.1-mean_median_mode
│       ├── Cargo.toml
│       └── src
│           └── main.rs       # Statistical calculations
├── 5-cryptographic_algorithms
│   └── 5.1-caeser_cipher
│       ├── Cargo.toml
│       └── src
│           └── main.rs       # Caesar cipher implementation
├── 6-port_sniffer
│   ├── Cargo.toml
│   └── src
│       └── main.rs           # Port scanning program
├── 7-io
│   ├── Cargo.toml
│   ├── src
│   │   ├── lib.rs           # I/O library module
│   │   └── main.rs          # File I/O with mutex and async examples
│   └── testing
│       ├── README.md         # I/O testing documentation
│       ├── hello.txt        # Sample text file
│       └── io               # Directory for I/O test files
└── README.md                    # Project overview (this file)
```

## Important Note
To ensure compatibility with Cargo, rename project directories by removing numeric prefixes (e.g., change `2-guessing_game` to `guessing_game`, `3.1-temperature_conversion` to `temperature_conversion`). Update the corresponding `Cargo.toml` file's `[package]` section to reflect the new name:
```toml
[package]
name = "guessing_game"  # Update to match renamed directory
version = "0.1.0"
edition = "2021"
```

## Overview
The projects progress from fundamental to advanced Rust concepts:
- **Fundamentals**: Ownership, borrowing, and lifetimes ensure memory safety without a garbage collector (e.g., in `7-io` for file handling).
- **Algorithms**: Basic algorithms like Fibonacci sequence (`3.2-nth_fibbonacci_number`) and statistical computations (`4.1-mean_median_mode`) showcase Rust's performance.
- **Collections**: Use of `Vec` and `HashMap` in `4.1-mean_median_mode` for data manipulation.
- **Error Handling**: Robust use of `Result` and `Option` types (e.g., in `6-port_sniffer` for network operations).
- **Advanced Features**: The `7-io` project demonstrates advanced Rust concepts like mutexes for thread-safe file access and async programming for non-blocking I/O operations.
Each project leverages Rust's zero-cost abstractions and compile-time checks for reliable, efficient code.

## Prerequisites
- Rust 1.70 or later
- Cargo (included with Rust)

## Setup
1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust-projects
   ```
3. Rename project directories to remove numeric prefixes (e.g., `2-guessing_game` to `guessing_game`) and update `Cargo.toml` files accordingly.

## Usage
1. Navigate to a project directory (after renaming):
   ```bash
   cd guessing_game
   ```
2. Build and run:
   ```bash
   cargo run
   ```

## Project Descriptions
- **1-hello_cargo**: A basic "Hello, World!" program introducing Rust and Cargo.
- **2-guessing_game**: A game where users guess a random number, demonstrating input handling and control flow.
- **3.1-temperature_conversion**: Converts temperatures between Celsius and Fahrenheit, showcasing arithmetic and user input.
- **3.2-nth_fibbonacci_number**: Generates the nth Fibonacci number, illustrating loops or recursion.
- **4.1-mean_median_mode**: Computes mean, median, and mode of a number list, using collections like `Vec` and `HashMap`.
- **5.1-caeser_cipher**: Implements a Caesar cipher for text encryption/decryption, focusing on string manipulation.
- **6-port_sniffer**: A port scanning program, demonstrating network programming and error handling.
- **7-io**: Demonstrates file I/O operations with a library module (`lib.rs`), including advanced features like mutexes for thread safety and async programming for non-blocking I/O.

## Building and Running
To build all projects (after renaming directories):
```bash
find . -name "Cargo.toml" -execdir cargo build \;
```
To run a specific project:
```bash
cd <renamed-project-directory>
cargo run
```

## Notes
- Rename directories to remove numeric prefixes for Cargo compatibility.
- The `7-io` project includes a `testing` directory with sample files for I/O operations.
- Ensure Rust is updated:
  ```bash
  rustup update
  ```
