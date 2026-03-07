use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg( long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" | "yml" => Ok(OutputFormat::Yaml),
            "toml" => Err(anyhow::anyhow!("To implement format: toml")),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
