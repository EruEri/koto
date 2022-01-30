use std::process::exit;

use clap::StructOpt;
use command::{Main, run_list};

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
