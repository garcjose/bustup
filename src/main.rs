mod cli;
use clap::Parser;

use crate::cli::Bustup;

fn main() -> anyhow::Result<()> {
    let args = Bustup::parse();
    todo!("Implement bustup");
    Ok(())
}
