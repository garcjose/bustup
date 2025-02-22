use clap::ArgMatches;

pub struct Ctx {
    pub toolchain: String,
}

impl Ctx {
    pub fn from_update(args: &ArgMatches) -> Self {
        Self {
            toolchain: args.get_one::<String>("toolchain").unwrap().to_string(),
        }
    }
}
