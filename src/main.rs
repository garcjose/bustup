mod cli;
mod context;
use clap::Parser;

use crate::cli::Bustup;
use crate::cli::Cmd;
use crate::context::Ctx;

fn main() -> anyhow::Result<()> {
    let args = Bustup::parse();

    let mut ctx = Ctx::default();

    let cmd: &dyn Cmd = &args;
    cmd.walk_exec(&mut ctx)
}
