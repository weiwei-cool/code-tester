---

# code-tester

A tool to check your algorithm.

[![Build Status](https://img.shields.io/badge/build-passing-green)](https://github.com/StelaHaveno/code-tester)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`code-tester` is a simple and powerful CLI tool for testing algorithms. It allows you to run an executable file, optionally provide input and expected output files, and verify correctness within a time limit.

---

## 📦 Features

- Run your compiled algorithm binary
- Provide custom input and expected output files
- Set time limits for execution
- Support for no-input mode
- Cross-platform support (Linux, macOS, Windows)

---

## 🚀 Installation

### From GitHub Release (Recommend)

Download the file you can use.

### From Source

```bash
git clone https://github.com/StelaHaveno/code-tester.git
cd code-tester
cargo build --release
```

The binary will be available at `target/release/code-tester`.

---

## 🧪 Usage

```bash
code-tester [OPTIONS] <SUBCOMMAND>
```

### Subcommands

#### `test`

Test if the algorithm is right.

```bash
code-tester test <file> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `<file>` | Path to the algorithm's executable file |
| `-i`, `--input-file <PATH>` | Path to the input file |
| `-o`, `--out-file <PATH>` | Expected output file (also referred to as the answer) |
| `-t`, `--time-limit <MS>` | Upper bound of algorithm runtime in milliseconds |
| `--no-input` | The algorithm has no input |

> ⚠️ You cannot use both `--input-file` and `--no-input` together.

---

## ✅ Example

```bash
code-tester test ./my_algorithm \
    --input-file ./in.txt \
    --out-file ./ans.txt \
    --time-limit 1000
```

This command:
- Runs the algorithm located at `./my_algorithm`
- Feeds it input from `./in.txt`
- Compares the output with `./ans.txt`
- Fails if execution exceeds 1000 ms

---

## 🛠️ Development

To build and run tests:

```bash
cargo build
cargo test
```

To run the tool locally:

```bash
cargo run -- test ./your_executable --input-file ./in.txt --out-file ./ans.txt
```

---

## 📄 License

This project is licensed under the [MIT License](License).

---