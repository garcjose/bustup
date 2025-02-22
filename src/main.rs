mod cli;
mod context;

use crate::cli::cmds::update;
use crate::context::Ctx;
use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    let args = cli::build().get_matches();
    match args.subcommand() {
        Some(("update", args)) => {
            let ctx = Ctx::from_update(args);
            update::run(&ctx)?
        }
        _ => todo!("implement other subcommands"),
    };
    Ok(())
}
