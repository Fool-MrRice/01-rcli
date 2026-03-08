use clap::{Parser, Subcommand, ValueEnum};

/// 文件签名命令行工具格式：
/// rcli sign [OPTIONS]
///
/// 选项：
///  --sign                生成签名模式
///   -i, --input <FILE>    输入文件（- 表示 stdin）
///   -o, --output <FILE>   输出文件（- 表示 stdout）
///   -k, --key <KEY>       密钥（字符串或 @文件路径）
///   -f, --format <FORMAT> 输出格式（hex 或 base64，默认 hex）
///
///  --verify              验证签名模式
///   -i, --input <FILE>    输入文件（- 表示 stdin）
///   -o, --output <FILE>   输出文件（- 表示 stdout）
///   -k, --key <KEY>       密钥（字符串或 @文件路径）
///   -f, --format <FORMAT> 输出格式（hex 或 base64，默认 hex）
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SignFormat {
    Hex,    // 十六进制输出（默认）
    Base64, // Base64 输出
}

#[derive(Debug, Subcommand)]
pub enum SignCommand {
    /// 生成签名
    Sign {
        /// 输入文件（- 表示 stdin）
        #[arg(short, long, default_value = "-")]
        input: String,

        /// 输出文件（- 表示 stdout）
        #[arg(short, long, default_value = "-")]
        output: String,

        /// 密钥（字符串或 @文件路径）
        #[arg(short, long)]
        key: String,

        /// 输出格式
        #[arg(short, long, value_enum, default_value = "hex")]
        format: SignFormat,
    },

    /// 验证签名
    Verify {
        /// 输入文件（- 表示 stdin）
        #[arg(short, long, default_value = "-")]
        input: String,

        /// 密钥（字符串或 @文件路径）
        #[arg(short, long)]
        key: String,

        /// 签名值（字符串或 @文件路径）
        #[arg(short, long)]
        signature: String,

        /// 签名格式
        #[arg(short, long, value_enum, default_value = "hex")]
        format: SignFormat,
    },
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[command(subcommand)]
    pub cmd: SignCommand,
}
impl From<SignFormat> for &'static str {
    fn from(val: SignFormat) -> Self {
        match val {
            SignFormat::Hex => "hex",
            SignFormat::Base64 => "base64",
        }
    }
}
// 需要实现 Display，或者使用 format.to_string()
impl std::fmt::Display for SignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignFormat::Hex => write!(f, "hex"),
            SignFormat::Base64 => write!(f, "base64"),
        }
    }
}
