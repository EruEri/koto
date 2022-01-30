use std::process::exit;

use clap::StructOpt;
use command::{Main, run_artist};
use sql::{Date, ArtistDB};

mod spotify;
mod command;
mod sql;


#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    // let client_id = std::env::var("CLIENT_ID").unwrap();
    // let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    // let token = Token::new(client_id.as_str(), client_secret.as_str()).await.unwrap();
    // let spotify = Spotify::new(&token);
    // //let search = spotify.search("Kitamura eri", vec![SpotifySearchType::Artist], None, None, None, None).await;
    // // println!("{:?}", search)
    // //let add_artist = Main::parse();

    // let artist = spotify.artist("5pjjlQXYjoMFWdjdKOre9s").await;
    // println!("{:?}", artist);


    let main = Main::parse();

    match main.artist {
        None => exit(0),
        Some(sub) => {
            match sub {
                command::Subcommands::Artist { artist_sub } => {
                    let _ = run_artist(artist_sub).await;
                },
            }
        },
        
    }
}
