use clap::Args;

/// add a target
#[derive(Clone, Args)]
pub struct BustupTargetAdd {
    /// target to add
    #[arg(default_value = "default")]
    pub target: String,
}
