use anyhow::Result;
use clap::Parser;

use rcli::process_csv;
use rcli::{Opts, Subcommand};

/// rcli-01 csv -i input.csv -o output.json --header -d ','
fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
