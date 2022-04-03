use std::{ffi::CStr, os::raw::c_char, path::PathBuf, process::exit};

use clap::StructOpt;
use command::{run_init, run_list, run_search, Main, run_edit};


mod command;
mod spotify;
mod util;
mod sql;

#[tokio::main]
async fn main() {
    let mut app_path = app_dir_pathbuf();
    app_path.push(".env");
    let _ = dotenv::from_path(app_path);
    let _ = dotenv::dotenv();
    if let None = std::env::var("CLIENT_ID").ok() {
        println!("CLIENT_ID key not found\nYou should maybe run koto init");
        exit(1)
    }
    if let None = std::env::var("CLIENT_SECRET").ok() {
        println!("CLIENT_SECRET key not found\nYou should maybe run koto init");
        exit(1)
    }
    // let client_id = std::env::var("CLIENT_ID").unwrap();
    // let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    // let token = crate::spotify::Token::new(client_id.as_str(), client_secret.as_str()).await.unwrap();
    // let spotify = spotify::Spotify::new(&token);
    // let search = spotify.search("LiSA", vec![spotify::SpotifySearchType::Track, spotify::SpotifySearchType::Artist], None, Some(10), None, None).await;
    // println!("{:?}", token);
    //println!("{:?}", search);
    let main = Main::parse();

    match main.subcommand {
        None => {
            println!("No sub");
            exit(0)
        }
        Some(sub) => match sub {
            command::Subcommands::List {
                delete,
                add,
                update,
                full,
                id,
            } => {
                let _ = run_list(delete, add, update, full, id).await;
            }
            command::Subcommands::Search {
                search_subcommand,
                artist,
                album,
                track,
                market,
                limit,
                offset,
                item,
            } => {
                let _ = run_search(search_subcommand, artist, album, track, market, limit, offset, item.unwrap_or("".into())).await;
            }
            command::Subcommands::Init {
                client_id,
                client_secret,
                force,
            } => {
                if let None = run_init(client_id, client_secret, force) {
                    println!("Unable to set the client credentials")
                }
            }
            command::Subcommands::Edit {
                file_type,
                title,
                artist,
                album,
                artist_album,
                year,
                bpm,
                track_position,
                images,
                output,
                file,
            } => {
                let _ = run_edit(file_type, title, artist, album, artist_album, year, bpm, track_position, images, output, file).await;

            },
        },
    }
}

extern "C" {
    fn get_home_dir() -> *mut c_char;
}

fn home_dir<'a>() -> &'a CStr {
    unsafe { CStr::from_ptr(get_home_dir()) }
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
