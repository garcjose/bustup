use clap::Args;

/// list targets
#[derive(Clone, Args)]
pub struct BustupTargetList {
    /// only list installed targets
    #[arg(short, long)]
    pub installed: bool,
}

impl BustupTargetList {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("listing targets...");
        Ok(())
    }
}
