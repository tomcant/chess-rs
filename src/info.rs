pub fn name() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

pub fn author() -> String {
    env!("CARGO_PKG_AUTHORS").to_string()
}
