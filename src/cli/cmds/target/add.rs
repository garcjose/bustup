use clap::Args;

use crate::cli::Cmd;
use crate::context::Ctx;

/// add a target
#[derive(Clone, Args)]
pub struct BustupTargetAdd {
    /// target to add
    #[arg(default_value = "default")]
    pub target: String,
}

impl Cmd for BustupTargetAdd {
    fn run(&self, ctx: &mut Ctx) -> anyhow::Result<()> {
        println!(
            "adding the {} target to the {} toolchain",
            self.target,
            ctx.target_toolchain.as_ref().unwrap()
        );
        Ok(())
    }
}
