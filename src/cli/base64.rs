use crate::cli::verify_input_file;
use clap::Parser;
use std::{fmt, str::FromStr};
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Decode,
    Encode,
}

#[derive(Debug, Parser)]
pub struct Base64Opts {
    #[arg( long, value_parser = parse_format, default_value = "decode")]
    pub format: Base64Format,

    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,

    #[arg(short, long,value_parser = verify_input, default_value = "-")]
    pub output: String,
}

fn parse_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "decode" => Ok(Base64Format::Decode),
            "encode" => Ok(Base64Format::Encode),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Decode => "decode",
            Base64Format::Encode => "encode",
        }
    }
}
fn verify_input(source: &str) -> Result<String, &'static str> {
    if source == "-" {
        Ok(source.to_string())
    } else {
        verify_input_file(source)
    }
}
