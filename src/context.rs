pub struct Ctx {
    pub toolchain: Option<String>,
    pub force: bool,
    pub target: Option<String>,
    pub installed: bool,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            toolchain: Some("default".to_string()),
            force: false,
            target: Some("target".to_string()),
            installed: false,
        }
    }
}
