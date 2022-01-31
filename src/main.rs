use std::{process::exit, os::raw::c_char, ffi::CStr, path::PathBuf};

use clap::StructOpt;
use command::{Main, run_list};

mod spotify;
mod command;
mod sql;

#[tokio::main]
async fn main() {
    let _ = dotenv::from_path(app_dir() + "/.env");
    // let client_id = std::env::var("CLIENT_ID").unwrap();
    // let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    // let token = crate::spotify::Token::new(client_id.as_str(), client_secret.as_str()).await.unwrap();
    // let spotify = spotify::Spotify::new(&token);
    // let search = spotify.search("LiSA", vec![spotify::SpotifySearchType::Track, spotify::SpotifySearchType::Artist], None, Some(10), None, None).await;
    // println!("{:?}", token);
    //println!("{:?}", search);
    let main = Main::parse();

    match main.subcommand {
        None => {println!("No sub"); exit(0)},
        Some(sub) => {
            match sub {
                command::Subcommands::List { delete, add, update, full, id } => {let _ = run_list(delete, add, update, full, id).await;},
            }
        },
        
    }
}

extern "C" {
    fn get_home_dir() -> *mut c_char;
}

fn home_dir<'a>() -> &'a CStr {
    unsafe {
        CStr::from_ptr(get_home_dir())
    }
}
pub fn app_dir() -> String {
    let mut pathbuf = PathBuf::from(home_dir().to_str().unwrap());
    pathbuf.push(".koto");
    pathbuf.to_str().unwrap().to_string()
}
pub fn app_dir_pathbuf() -> PathBuf {
    let mut pathbuf = PathBuf::from(home_dir().to_str().unwrap());
    pathbuf.push(".koto");
    pathbuf
}