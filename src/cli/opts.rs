use clap::Parser;

use crate::cli::{Base64Opts, CsvOpts, GenPassOpts, SignOpts};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate random password")]
    GenPass(GenPassOpts),
    #[command(name = "base64", about = "Encode or decode base64")]
    Base64(Base64Opts),
    #[command(name = "sign", about = "Sign or verify data")]
    Sign(SignOpts),
}
