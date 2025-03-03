mod cli;
use clap::Parser;

use crate::cli::Bustup;

fn main() -> anyhow::Result<()> {
    let args = Bustup::parse();
    match args.cmd {
        cli::BustupCmd::Update(update) => update.run()?,
        cli::BustupCmd::Target(target) => target.run()?,
    }
    Ok(())
}
