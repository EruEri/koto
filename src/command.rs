use std::{fs::OpenOptions, io::{BufReader, Seek, SeekFrom, Write, BufRead}};

use clap::{Parser, Subcommand};

use crate::{spotify::{Token, Spotify}, sql::ArtistDB};

const ARTIST_FILE : &'static str = "/Users/ndiaye/Documents/Lang/Rust/Cargo/exec/new_song/.artists";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Main {
    #[clap(subcommand)]
    pub artist : Option<Subcommands>
}


#[derive(Subcommand)]
pub enum Subcommands {
    /// Add an artist to artist list
    Artist {

        #[clap(subcommand)]
        artist_sub : ArtistSubcommands
    }
}

// #[derive(Clone, Copy, ArgEnum)]
// pub enum ArtistFlag {
//     Delete,
//     List
// }

pub async fn run_artist(sub : ArtistSubcommands) -> Option<()>{

    match sub {
        ArtistSubcommands::Update { names, ids } => {
            let client_id = std::env::var("CLIENT_ID").unwrap();
            let client_secret = std::env::var("CLIENT_SECRET").unwrap();
            let token = crate::spotify::Token::new(client_id.as_str(), client_secret.as_str()).await.unwrap();
            let spotify = Spotify::new(&token);
            run_artist_update(names, ids, &spotify).await
        },
        ArtistSubcommands::List { all_info, name, id } => {
            run_artist_list(all_info, name, id)
        },
        ArtistSubcommands::Add { id, delete, name } => {
            run_artist_add(id, delete, name).await
        },
    }

}

pub async fn run_artist_add(id : bool, delete : bool, name : String ) -> Option<()>{
    let name = if id {
        let client_id = std::env::var("CLIENT_ID").unwrap();
        let client_secret = std::env::var("CLIENT_SECRET").unwrap();
        let token = Token::new(client_id.as_str(), client_secret.as_str()).await.unwrap();
        let spotify = Spotify::new(&token);
        let artist = spotify.artist(name.as_str()).await?;
        artist.name
    }else { name };
    let file = OpenOptions::new().create(true).truncate(false).read(true).write(true).open(ARTIST_FILE).ok()?;

    if !delete {
        let reader = BufReader::new(file);
        let names = reader.lines().filter_map(|n| n.ok()).collect::<Vec<String>>();
        if names.contains(&name) { println!("{} : Already exist", name); return None;}
        let mut file = OpenOptions::new().create(false).write(true).open(ARTIST_FILE).ok()?;
        file.seek(SeekFrom::End(0)).ok()?;
        file.write(name.as_bytes()).ok()?;
        file.write("\n".as_bytes()).ok()?;
    }else {
        let reader = BufReader::new(file);
        let mut names = reader.lines().filter_map(|n| n.ok()).collect::<Vec<String>>();
        names.retain(|fname| fname != &name);
        let mut file = OpenOptions::new().create(false).truncate(true).write(true).open(ARTIST_FILE).ok()?;
        names.into_iter().for_each(|n| { 
            let _ = file.write(format!("{}\n", n).as_bytes()); 
         });
         println!("Done")
    }

    Some(())
}

pub async fn run_artist_update(names : Option<Vec<String>>, ids : Option<Vec<String>>, spotify : &Spotify) -> Option<()> {
    let (mut artists, connection) = ArtistDB::fetch_all().unwrap_or_else(|| panic!("Unable to fetch into the database"));
    if let Some(name) = names {
        artists.retain(|artist| name.contains(&artist.artist_name));
    }
     if let Some(id) = ids {
         artists.retain(|artist| id.contains(&artist.artist_spotify_id));
    }
    println!();
    println!("------------------------------");
    println!("-------- New realease --------");
    println!("------------------------------");
    println!("");
    for artist in artists.iter_mut() {
        let updated  = artist.update(&spotify, &connection).await;
        if let Some(updated) = updated {
            if updated {
                println!("\n");
                println!("{}", artist.default_format());
                (0..3).into_iter().for_each(|_| println!("------------------"));
            }
        }else {
            println!("An error occured for : {}", artist.artist_name)
        }
    }
    println!("Done");
    Some(())
}

pub fn run_artist_list(all_info : bool, name : Option<Vec<String>>, id : Option<Vec<String>>) -> Option<()>{
    let mut artists = ArtistDB::fetch_all()?.0;
    if let Some(name) = name {
       artists.retain(|artist| name.contains(&artist.artist_name));
    }
    if let Some(id) = id {
        artists.retain(|artist| id.contains(&artist.artist_spotify_id));
    }

     for artist in artists {
         
         println!("\n");
         let artist_str = if all_info { artist.full_format() } else { artist.default_format() };
         println!("{}", artist_str);
         (0..3).into_iter().for_each(|_| println!("------------------"));
     }


     Some(())
}


#[derive(Subcommand)]

pub enum  ArtistSubcommands {
    /// Display all the artists 
    List {
        /// Show all the information about the artist
        #[clap(short,long)]
        all_info : bool,

        /// Filter with selected name
        #[clap(short, long)]
        name : Option<Vec<String>>,

        /// Filter with selected spotify id
        #[clap(short, long)]
        id : Option<Vec<String>>
    },
    /// Add or Remove artist from the list
    Add {
        /// Find an artist with spotify id
        #[clap(short, long)]
        id : bool,
        /// Delete an artist to the list 
        #[clap(short, long)]
        delete : bool,

        name : String,
    }
    ,
    /// Update the artist lastest song
    Update {
        /// Update for a specific artist
        #[clap(short, long)]
        names : Option<Vec<String>>,

        /// Update for a specific artist id
        #[clap(short, long)]
        ids : Option<Vec<String>>
    }
}