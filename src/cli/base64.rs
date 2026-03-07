use crate::cli::verify_input_file;
use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Debug, Clone, Copy, ValueEnum)] // ✅ 用 ValueEnum 简化
pub enum Base64Format {
    Decode,
    Encode,
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::Decode => write!(f, "decode"),
            Base64Format::Encode => write!(f, "encode"),
        }
    }
}

#[derive(Debug, Parser)]
pub struct Base64Opts {
    #[arg(long, value_enum, default_value = "decode")] // ✅ 直接用 value_enum
    pub format: Base64Format,

    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub output: String,
}

// ✅ 简化：统一错误类型，支持动态错误信息
fn verify_input(source: &str) -> Result<String, String> {
    if source == "-" {
        Ok(source.to_string())
    } else {
        verify_input_file(source).map_err(|e| e.to_string())
    }
}
