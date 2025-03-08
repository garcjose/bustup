mod cmds;

use clap::{Parser, Subcommand};

use crate::cli::cmds::*;
use crate::context::Ctx;

/// not rustup
#[derive(Parser)]
pub struct Bustup {
    #[command(subcommand)]
    pub cmd: BustupCmd,
}

#[enum_delegate::implement(Cmd)]
#[derive(Clone, Subcommand)]
pub enum BustupCmd {
    Update(update::BustupUpdate),
    Target(target::BustupTarget),
}

#[enum_delegate::register]
pub trait Cmd {
    fn update_ctx(&self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        Ok(())
    }
    fn run(&self, _ctx: &mut Ctx) -> anyhow::Result<()> {
        Ok(())
    }
    fn next_cmd(&self) -> Option<&dyn Cmd> {
        None
    }
}

impl<'a> dyn Cmd + 'a {
    pub fn walk_exec(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        self.update_ctx(ctx)?;
        self.run(ctx)?;
        if let Some(c) = self.next_cmd() {
            return c.walk_exec(ctx);
        }
        Ok(())
    }
}

impl Cmd for Bustup {
    fn next_cmd(&self) -> Option<&dyn Cmd> {
        Some(&self.cmd)
    }
}
