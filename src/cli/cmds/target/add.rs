use crate::{cli::Cmd, context::Ctx};
use anyhow::Result;
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct BustupTargetAdd;
impl Cmd for BustupTargetAdd {
    fn update_ctx(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.target = args.get_one::<String>("target").cloned();
        Ok(())
    }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Adding the {} target to the {} toolchain",
            ctx.target.as_ref().unwrap(),
            ctx.toolchain.as_ref().unwrap(),
        );
        Ok(())
    }
}

pub fn build() -> Command {
    Command::new("add").about("add a target").arg(
        Arg::new("target")
            .help("The target to add")
            .action(ArgAction::Set),
    )
}
