# CodeTester Documentation

[中文文档](README_ZH.md)

## 📌 Introduction
`code-tester` is a Rust-based algorithm validation tool that automates testing to help developers verify algorithm correctness and performance. It supports single-case testing, batch testing, and runtime limits.

## 🚀 Installation

### crates.io
```bash  
cargo install code-tester  
```  

### GitHub Release
Direct download available

## 📦 Command Overview
### `test` Command
Validates algorithm correctness and performance.

**Syntax**:
```bash  
code-tester test [OPTIONS] <FILE>  
```  

### Arguments
| Argument | Type | Description |  
|---------|------|-------------|  
| `<FILE>` | Required | Path to the algorithm executable |  

### Options
| Option | Shortcut | Type | Description |  
|-------|----------|------|-------------|  
| `--input-file` | `-i` | File Path | Specify input file path |  
| `--ans-file` | `-a` | File Path | Reference answer/output file |  
| `--time-limit` | `-t` | Number (ms) | Maximum execution time in milliseconds |  
| `--data` | `-d` | Path | Test case directory/archive (supports `.in/.ans` pairs, folders, or ZIP files) |  
| `--no-input` | - | Flag | Indicates no input required |  

## 🔍 Examples
### Single Case Testing
```bash  
# Test my_algo with input/answer files and 200ms timeout  
code-tester test my_algo --input-file input.txt --ans-file answer.txt --time-limit 200  
```  

### Batch Testing
```bash  
# Use test directory (must contain test_cases1.in/test_cases1.ans etc.)  
code-tester test my_algo --data test_cases/  

# ZIP file support  
code-tester test my_algo --data examples.zip  
```  

### No-Input Testing
```bash  
# For input-free algorithms  
code-tester test my_algo --no-input --ans-file answer.txt  
```  

## ⚠️ Important Notes
1. **Time Unit**: `--time-limit` uses milliseconds (e.g., 200ms = 0.2s)
2. **Archive Requirements**: Ensure complete test file pairs after extraction
3. **Input Conflict**: `--no-input` cannot coexist with `--input-file`
4. **Test Case Naming Rules**:
    - Must contain matched `example<number>.in` and `example<number>.ans` files
    - **Folder name must match file prefix**:
        - ✅ Valid Examples:  
          `a/a1.in` + `a/a1.ans`  
          `b.zip/b1.in` + `b.zip/b1.ans`
        - ❌ Invalid Examples:  
          `a/b1.in` (folder-name mismatch)  
          `test/test.in` (missing numeric index)

## 📊 Version Info
```bash  
code-tester --version  
```  

## 💡 Developer Guide
```bash  
# Build from source  
git clone https://github.com/weiwei-cool/code-tester  
cd code-tester  
cargo build --release  
```  

## 📝 Contributing
We welcome PRs! Follow semantic versioning and include test cases for new features.

## 📄 License
MIT License

---