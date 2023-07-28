use xdg::BaseDirectories;

pub const KOTO_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const KOTO_NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn koto_base_dir() -> BaseDirectories {
    xdg::BaseDirectories::with_prefix(KOTO_NAME)
        .unwrap_or_else(|e| panic!("Cannot create xdg dirs: {}", e))
}
