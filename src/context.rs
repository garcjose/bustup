#[derive(Default)]
pub struct Ctx {
    pub toolchain: Option<String>,
    pub force: bool,
    pub target: Option<String>,
    pub installed: bool,
}
