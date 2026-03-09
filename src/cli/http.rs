use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
// #[derive(Debug, Clone, Copy)]
// pub enum OutputFormat {
//     Json,
//     Yaml,
// }

fn verify_input_dir(dir: &str) -> Result<PathBuf, &'static str> {
    if Path::new(dir).exists() {
        Ok(PathBuf::from(dir))
    } else {
        Err("Directory does not exist")
    }
}

#[derive(Debug, Parser)]
pub struct HttpOpts {
    #[command(subcommand)]
    pub cmd: HttpCommand,
}

#[derive(Debug, Subcommand)]
pub enum HttpCommand {
    #[command(about = "Start an HTTP server")]
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,

        #[arg(short, long,value_parser = verify_input_dir,default_value = ".")]
        dir: PathBuf,
    },
}
