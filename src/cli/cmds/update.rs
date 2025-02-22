use crate::context::Ctx;
use anyhow::Result;

pub fn run(ctx: &Ctx) -> Result<()> {
    println!("Rupdatting toolchain... {}", ctx.toolchain);
    Ok(())
}
