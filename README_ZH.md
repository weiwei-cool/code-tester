---

# code-tester

一个用于检查算法正确性的工具。

[![Build Status](https://img.shields.io/badge/build-passing-green)](https://github.com/weiwei-cool/code-tester)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`code-tester` 是一个简单而强大的命令行工具，用于测试你编写的算法程序。你可以运行可执行文件、提供输入和预期输出文件，并在指定的时间限制内验证其正确性。

---

## 📦 功能特性

- 运行你的编译后的算法程序或者python代码
- 提供自定义的输入和预期输出文件
- 设置运行时间上限
- 支持无输入模式
- 跨平台支持（Linux、macOS、Windows）

---

## 🚀 安装方式

### 从 GitHub 安装（推荐）

下载您能使用的版本即可。

### 从源码安装

```bash
git clone https://github.com/StelaHaveno/code-tester.git
cd code-tester
cargo build --release
```

编译完成后，可执行文件位于 `target/release/code-tester`。

---

## 🧪 使用方法

```bash
code-tester [OPTIONS] <SUBCOMMAND>
```

### 子命令

#### `test`

用于测试算法是否正确。

```bash
code-tester test <file> [OPTIONS]
```

| 参数 | 描述 |
|------|------|
| `<file>` | 算法可执行文件路径 |
| `-i`, `--input-file <PATH>` | 输入文件路径 |
| `-o`, `--out-file <PATH>` | 预期输出文件路径（即标准答案） |
| `-t`, `--time-limit <MS>` | 最大允许运行时间（毫秒） |
| `--no-input` | 表示该算法不需要输入 |

> ⚠️ 注意：不能同时使用 `--input-file` 和 `--no-input`。

---

## ✅ 示例

```bash
code-tester test ./my_algorithm \
    --input-file ./in.txt \
    --out-file ./ans.txt \
    --time-limit 1000
```

该命令会：
- 执行 `./my_algorithm`
- 从 `./in.txt` 获取输入
- 将输出与 `./ans.txt` 中的标准答案进行比较
- 如果执行时间超过 1000 毫秒则判定为失败

当然也能运行python程序：
```bash
code-tester test ./my_code.py \
    --input-file ./in.txt \
    --out-file ./ans.txt \
    --time-limit 1000
```

---

## 🛠️ 开发构建

构建项目并运行测试：

```bash
cargo build
cargo test
```

本地运行项目：

```bash
cargo run -- test ./your_executable --input-file ./in.txt --out-file ./ans.txt
```

---

## 📄 协议

本项目采用 [MIT License](License) 开源协议。

---