mod cli;

pub use cli::{Base64Format, Base64Opts, GenPassOpts, Opts, Subcommand};
mod process;
pub use process::*;
