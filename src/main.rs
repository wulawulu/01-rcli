// rcli csc -i input.csv -o output.json

use clap::Parser;
use rcli::{process_csv, process_gen_pass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_gen_pass(
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
                opts.length,
            )?;
        }
    }

    Ok(())
}
