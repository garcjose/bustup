use crate::{cli::Cmd, context::Ctx};
use anyhow::Result;
use clap::ArgMatches;

pub struct BustupUpdate;
impl Cmd for BustupUpdate {
    fn update_ctx(&self, args: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.toolchain = args.get_one::<String>("toolchain").cloned();
        ctx.force = args.get_flag("force");
        Ok(())
    }

    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        println!(
            "Updating {} toolchain... {}",
            ctx.toolchain.as_ref().unwrap(),
            if ctx.force { "(forced)" } else { "" }
        );
        Ok(())
    }
}
