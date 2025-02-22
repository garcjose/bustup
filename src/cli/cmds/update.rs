use anyhow::Result;
use clap::ArgMatches;

pub fn run(args: &ArgMatches) -> Result<()> {
    println!(
        "Rupdatting toolchain... {}",
        args.get_one::<String>("toolchain").unwrap()
    );
    Ok(())
}
