# RCLI

RCLI 是一个用 Rust 编写的命令行工具，用于处理和转换 CSV 文件。

## 功能特性

- 📊 **CSV 转换**: 将 CSV 文件转换为 JSON 格式
- ⚙️ **灵活配置**: 支持自定义分隔符和 Header 选项
- 🚀 **高性能**: 利用 Rust 的性能优势实现快速处理
- ✅ **类型安全**: 使用 serde 进行数据序列化和反序列化

## 环境要求

- Rust 1.70 或更高版本
- Cargo

## 安装

```bash
git clone <repository-url>
cd rcli-01
cargo build --release
```

## 使用方法

### 基本用法

```bash
# 将 CSV 文件转换为 JSON
cargo run -- csv -i input.csv -o output.json
```

### 命令行选项

```
rcli csv [OPTIONS]

OPTIONS:
  -i, --input <INPUT>              输入的 CSV 文件路径（必需）
  -o, --output <OUTPUT>            输出的 JSON 文件路径 [默认: output.json]
  --header <HEADER>                是否包含 header 行 [默认: true]
  -d, --delimiter <DELIMITER>      CSV 文件的分隔符 [默认: ',']
  -h, --help                       显示帮助信息
  -V, --version                    显示版本号
```

## 示例

```bash
# 使用默认配置（逗号分隔符）
cargo run -- csv -i assets/juventus.csv -o output.json

# 使用自定义分隔符
cargo run -- csv -i input.tsv -o output.json -d '\t'

# 禁用 header
cargo run -- csv -i input.csv -o output.json --header false
```

## 项目结构

```
├── src/
│   ├── main.rs          # 主程序入口
│   ├── lib.rs           # 库文件，导出主要接口
│   ├── opts.rs          # 命令行选项定义
│   └── process.rs       # CSV 处理逻辑
├── assets/              # 示例数据文件
│   └── juventus.csv
├── Cargo.toml           # 项目配置文件
└── README.md            # 本文件
```

## 依赖

- **clap**: 用于命令行参数解析
- **serde**: 数据序列化框架
- **serde_json**: JSON 序列化支持
- **csv**: CSV 处理库
- **anyhow**: 错误处理

## 示例数据

项目包含 `assets/juventus.csv` 作为示例数据，包含以下字段：

- Name: 球员名字
- Position: 位置
- DOB: 出生日期
- Nationality: 国籍
- Kit Number: 球衣号码

转换后的 JSON 输出示例：

```json
[
  {
    "Name": "Player Name",
    "Position": "Forward",
    "DOB": "1995-01-15",
    "Nationality": "Italy",
    "Kit Number": 7
  }
]
```

## 许可证

MIT License

## 作者

Mr.Rice
