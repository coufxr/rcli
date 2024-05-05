use clap::Parser;

use rcli::{CliOpts, CmdSubcommand, process_csv};

fn main() -> anyhow::Result<()> {
    let opts: CliOpts = CliOpts::parse();
    match opts.cmd {
        CmdSubcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
