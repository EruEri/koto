use xdg::BaseDirectories;

pub const KOTO_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const KOTO_NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn koto_base_dir() -> BaseDirectories {
    xdg::BaseDirectories::with_prefix(KOTO_NAME)
        .unwrap_or_else(|e| panic!("Cannot create xdg dirs: {}", e))
}


pub fn extend_env() {
    let koto_dir = koto_base_dir();
    let path = match koto_dir.find_config_file(".env") {
        Some(path) => path,
        None => {
            println!("No env file, You should maybe run {} init", KOTO_NAME);
            return;
        }
    };
    let _ = dotenv::from_path(path);
    let _ = dotenv::dotenv();
}

pub fn check_credential_exist() -> bool {
    if let None = std::env::var("CLIENT_ID").ok() {
        println!("CLIENT_ID key not found\nYou should maybe run koto init");
        return false;
    }
    if let None = std::env::var("CLIENT_SECRET").ok() {
        println!("CLIENT_SECRET key not found\nYou should maybe run koto init");
        return false;
    }

    return true;
}