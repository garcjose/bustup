mod cli;

use crate::cli::cmds::update;
use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    let args = cli::build().get_matches();
    match args.subcommand() {
        Some(("update", args)) => update::run(args)?,
        _ => todo!("implement other subcommands"),
    };
    Ok(())
}
