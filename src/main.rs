// use std::os::windows::process;

use anyhow::Result;
use clap::Parser;

use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve, process_sign,
    process_verify,
};
use rcli::{Base64Format, HttpCommand, Opts, SignCommand, Subcommand};

/// rcli-01 csv -i input.csv -o output.json --header -d ','
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化 tracing 子系统,用于记录日志，会将日志输出到标准错误流
    // 如何使用：
    // 1. 直接运行程序，默认级别为 info
    // 2. 设置 RUST_LOG 环境变量，例如 RUST_LOG=debug
    // 3. 在代码中使用 tracing 宏，例如 tracing::info!("hello world")

    tracing_subscriber::fmt().init();
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        Subcommand::Base64(opts) => match opts.format {
            Base64Format::Decode => {
                process_decode(&opts.input, &opts.output)?;
            }
            Base64Format::Encode => {
                process_encode(&opts.input, &opts.output)?;
            }
        },
        Subcommand::Sign(sign_opts) => match sign_opts.cmd {
            SignCommand::Sign {
                input,
                output,
                key,
                format,
            } => {
                process_sign(&input, &key, &output, format)?;
            }
            SignCommand::Verify {
                input,
                key,
                signature,
                format,
            } => {
                process_verify(&input, &key, &signature, format)?; // 注意参数
            }
        },
        Subcommand::Http(opts) => match opts.cmd {
            HttpCommand::Serve { port, dir } => {
                process_http_serve(port, dir).await?;
            }
        },
    }
    Ok(())
}
