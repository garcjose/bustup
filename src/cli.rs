mod cmds;

use clap::{Parser, Subcommand};

use crate::cli::cmds::*;

/// not rustup
#[derive(Parser)]
pub struct Bustup {
    #[command(subcommand)]
    pub cmd: BustupCmd,
}

#[derive(Clone, Subcommand)]
pub enum BustupCmd {
    Update(update::BustupUpdate),
    Target(target::BustupTarget),
}
