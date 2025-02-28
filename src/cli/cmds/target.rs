use crate::{cli::Cmd, context::Ctx};
use anyhow::Result;
use clap::ArgMatches;

pub struct BustupTarget;
impl Cmd for BustupTarget {
    fn update_ctx(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.toolchain = args.get_one::<String>("toolchain").cloned();
        Ok(())
    }

    fn next_cmd<'a>(&self, matches: &'a ArgMatches) -> Option<(Box<dyn Cmd>, &'a ArgMatches)> {
        match matches.subcommand() {
            Some(("add", args)) => Some((Box::new(add::BustupTargetAdd), args)),
            Some(("list", args)) => Some((Box::new(list::BustupTargetList), args)),
            Some(("remove", args)) => Some((Box::new(remove::BustupTargetRemove), args)),
            _ => None,
        }
    }
}

pub mod add;
pub mod list;
pub mod remove;
