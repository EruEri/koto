use std::{fs::OpenOptions, io::{BufReader, Seek, SeekFrom, Write, BufRead}};

use clap::{Parser, Subcommand, ArgEnum};

use crate::spotify::{Token, Spotify};

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
        /// Find an artist with spotify id
        #[clap(short, long)]
        id : bool,
        /// Delete an artist to the list 
        #[clap(short, long)]
        delete : bool,

        name : String,

        #[clap(subcommand)]
        artist_sub : Option<ArtistSubcommands>
    }
}

// #[derive(Clone, Copy, ArgEnum)]
// pub enum ArtistFlag {
//     Delete,
//     List
// }

pub async fn run_artist(id : bool, delete : bool, name : String, sub : Option<ArtistSubcommands>) -> Option<()>{
    if let Some(sub) = sub {
        match sub {
            ArtistSubcommands::List { all } => {
                todo!()
            },
            ArtistSubcommands::Update { name } => todo!(),
        }
    }
    let name = if id {
        let client_id = std::env::var("CLIENT_ID").unwrap();
        let client_secret = std::env::var("CLIENT_SECRET").unwrap();
        let token = Token::new(client_id.as_str(), client_secret.as_str()).await.unwrap();
        let spotify = Spotify::new(&token);
        let artist = spotify.artist(name.as_str()).await?;
        println!("{:?}", artist);
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
         })
    }

    Some(())
}


#[derive(Subcommand)]

pub enum  ArtistSubcommands {
    /// Display all the artists 
    List {
        /// Show all the information about the artist
        #[clap(long)]
        all : bool
    },
    /// Update the artist lastest song
    Update {
        /// Update for a specific artist
        #[clap(short, long)]
        name : Option<String>
    }
}