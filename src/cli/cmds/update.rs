use anyhow::Result;
use clap::Args;

/// update toolchains
#[derive(Clone, Args)]
pub struct BustupUpdate {
    /// toolchain to update
    #[arg(default_value = "default")]
    pub toolchain: String,

    /// forcibly update toolchain
    #[arg(short, long)]
    pub force: bool,
}

impl BustupUpdate {
    pub fn run(&self) -> Result<()> {
        println!(
            "updating {} toolchain...{}",
            if self.force { "forcibly" } else { "" },
            self.toolchain
        );
        Ok(())
    }
}
