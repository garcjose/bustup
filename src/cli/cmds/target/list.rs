use clap::Args;

use crate::cli::Cmd;
use crate::context::Ctx;

/// list targets
#[derive(Clone, Args)]
pub struct BustupTargetList {
    /// only list installed targets
    #[arg(short, long)]
    pub installed: bool,
}

impl Cmd for BustupTargetList {
    fn run(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        println!(
            "Listing {} targets for the {} toolchain",
            if self.installed { "installed" } else { "all" },
            ctx.target_toolchain.as_ref().unwrap()
        );
        Ok(())
    }
}
