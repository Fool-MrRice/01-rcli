// use std::os::windows::process;

use anyhow::Result;
use clap::Parser;

use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_sign, process_verify,
};
use rcli::{Base64Format, HttpCommand, Opts, SignCommand, Subcommand};

/// rcli-01 csv -i input.csv -o output.json --header -d ','
fn main() -> Result<()> {
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
        Subcommand::Http(opts) => {
            match opts.cmd {
                HttpCommand::Serve { port, dir } => {
                    println!("{:?}", (port, dir));
                    println!("serving at http://localhost:{}", port);
                    // todo: process_http(&opts)?;
                }
            }
        }
    }
    Ok(())
}
