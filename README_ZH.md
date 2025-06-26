# CodeTester 中文文档

## 📌 工具简介
`code-tester` 是基于 Rust 开发的算法验证工具，通过自动化测试帮助开发者快速验证算法程序的正确性和性能表现。支持单用例测试、批量测试、运行时长限制等功能。

## 🚀 安装方式

### crates.io
```bash
cargo install code-tester
```

### GitHub Release
直接下载即可

## 📦 命令说明
### `test` 命令
用于验证算法程序的正确性及性能

**基本语法**：
```bash
code-tester test [OPTIONS] <FILE>
```

### 参数说明
| 参数 | 类型 | 描述 |
|------|------|------|
| `<FILE>` | 必填 | 待测试算法的可执行文件路径 |

### 可选参数
| 选项 | 缩写 | 参数类型 | 描述 |
|------|------|----------|------|
| `--input-file` | `-i` | 文件路径 | 指定输入文件路径 |
| `--ans-file` | `-a` | 文件路径 | 标准答案文件路径（参考输出） |
| `--time-limit` | `-t` | 数值（ms） | 设置运行时长上限（单位：毫秒） |
| `--data` | `-d` | 文件/目录路径 | 指定测试用例集（支持`.in/.ans`文件组、文件夹或压缩包） |
| `--no-input` | 无 | 标志位 | 标记算法无需输入文件 |

## 🔍 使用示例
### 单用例测试
```bash
# 测试算法可执行文件 my_algo，指定输入/答案文件并设置200ms时间限制
code-tester test my_algo --input-file input.txt --ans-file answer.txt --time-limit 200
```

### 批量测试
```bash
# 使用测试用例目录（需包含 test_cases1.in/test_cases1.ans 等规范命名文件）
code-tester test my_algo --data test_cases/

# 使用压缩包测试（支持 zip 格式）
code-tester test my_algo --data examples.zip
```

### 无输入测试
```bash
# 针对不依赖输入的算法，直接比对输出结果
code-tester test my_algo --no-input --ans-file answer.txt
```

## ⚠️ 注意事项
1. **时间单位敏感**：`--time-limit` 参数单位为毫秒（如 200ms = 0.2秒）
2. **压缩包要求**：需确保解压后保留完整测试用例文件组
3. **输入冲突规避**：启用 `--no-input` 后不可同时指定 `--input-file`
4. **测试用例命名规范**：
    - 目录/压缩包内需包含配对的 `example<number>.in` 和 `example<number>.ans` 文件
    - **文件夹名需与文件前缀一致**：
        - ✅ 合法示例：  
          `a/a1.in` + `a/a1.ans`  
          `b.zip/b1.in` + `b.zip/b1.ans`
        - ❌ 非法示例：  
          `a/b1.in`（文件夹名与文件前缀不匹配）  
          `test/test.in`（未使用数字编号）

## 📊 版本信息
```bash
code-tester --version
```

## 💡 开发者指南
```bash
# 源码构建
git clone https://github.com/weiwei-cool/code-tester
cd code-tester
cargo build --release
```

## 📝 贡献说明
欢迎提交 PR 改进工具，请遵循语义化版本控制规范，并提供完整测试用例验证新增功能。

## 📄 许可证
MIT License
