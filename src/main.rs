use std::process::exit;

use clap::StructOpt;
use command::{Main, run_artist};

mod spotify;
mod command;
mod sql;


#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    // let client_id = std::env::var("CLIENT_ID").unwrap();
    // let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    // let token = crate::spotify::Token::new(client_id.as_str(), client_secret.as_str()).await.unwrap();
    // let spotify = spotify::Spotify::new(&token);
    // //let search = spotify.search("Kitamura eri", vec![SpotifySearchType::Artist], None, None, None, None).await;
    //  println!("{:?}", token);
    // let add_artist = Main::parse();
    
    // // let albums = spotify.artist_album("5pjjlQXYjoMFWdjdKOre9s".into(), vec![
    // //     spotify::SpotifyIncludeGroupe::Album,
    // // ], Some(10), None, None).await;
    // // if let Some(album) = albums {
    // //     println!("{:?}", album.items);
    // // }

    // //println!("{:?}", albums)
    // // let artist = spotify.artist("5pjjlQXYjoMFWdjdKOre9s").await?;
    // // println!("{:?}", artist);
    // let result = sql::ArtistDB::fetch_all().unwrap().0;
    // let mut asca_vec = result.into_iter().filter(|a| {
    //     a.artist_spotify_id == "5Qeyh2XKoITt1mlEVtzazC"
    // }).collect::<Vec<sql::ArtistDB>>();
    // let asca = asca_vec.first_mut().unwrap();

    // let x = asca.update(&spotify).await;
    // println!("{:?}", x);

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
