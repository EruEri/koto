use clap::ArgGroup;
use rusqlite::params;
use std::{process::exit, fs::OpenOptions, io::Write};

use clap::{Parser, Subcommand};

use crate::{
    spotify::{Spotify, SpotifySearchType},
    sql::ArtistDB, app_dir_pathbuf,
};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Main {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommands>,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// List the followed artists
    #[clap(group(
        ArgGroup::new("opt")
        .required(false)
        .args(&["delete", "add"])
        .conflicts_with("update")
        .conflicts_with("full")
    )) ]
    List {
        /// delete an artist to the list
        #[clap(short, long)]
        delete: Option<String>,
        /// add an artist to the list
        #[clap(short, long)]
        add: Option<String>,
        /// update the artist lastest album
        #[clap(short, long)]
        update: Option<Option<String>>,

        /// Display all the artist information
        #[clap(short, long)]
        full: bool,
        /// Filter with selected spotify id
        #[clap(short, long)]
        id: bool,
    },
    /// Search for an item
    Search {
        /// search for an artist
        #[clap(short, long)]
        artist : bool,
        /// search for an album
        #[clap(short = 'b', long)]
        album : bool,
        /// search for an track
        #[clap(short, long)]
        track : bool,

        /// market to look for
        #[clap(long)]
        market : Option<String>,

        /// limit the result
        /// MAX Value : 50
        #[clap(long)]
        limit : Option<u8>,

        /// offset the result
        #[clap(long)]
        offset : Option<u32>,



        /// search item
        item : String
    },
    /// Init koto with the spotify client credentials
    Init {
        /// Set the client id
        #[clap(long)]
        client_id : String,
        /// Set the client secret
        #[clap(long)]
        client_secret : String
    }
}

pub async fn run_list(
    delete: Option<String>,
    add: Option<String>,
    update: Option<Option<String>>,
    full: bool,
    id: bool,
) -> Option<()> {
    match (delete, add) {
        (Some(_), Some(_)) => unreachable!("delete and add cant be together in the same time"),
        (None, Some(add)) => run_list_modify(id, false, add).await,
        (Some(delete), None) => run_list_modify(id, true, delete).await,
        (None, None) => match update {
            None => run_list_show(full, None, false),
            Some(artist_opt) => run_list_update(artist_opt.map(|art| vec![art]), id).await,
        },
    }
}

pub async fn run_list_modify(id: bool, delete: bool, name: String) -> Option<()> {
    if !delete {
        let spotify = Spotify::init().await;
        let artist = ArtistDB::from_name(&name, id, &spotify).await?;
        match ArtistDB::insert_db(&artist){
            Ok(u) => {
                if u == 1 {
                    println!("Operation Succesed\n{} added to the database", name);
                }
                
            },
            Err(e) => {
                println!("An error occured \n{}", e);
                return None ;
            },
        }
    } else {
        let connection = ArtistDB::open().unwrap_or_else(|| panic!("Unable to open the database"));
        let field = if id { "artist_spotify_id" } else { "artist_name" };
        let sql_string = format!("DELETE FROM artist_table WHERE {} = ?1", field);
        match connection.execute(sql_string.as_str(), params![name]){
            Ok(size) => if size == 0 { println!("No artist deleted")} else { println!("{} artist deleted", size)},
            Err(_) => {
                println!("An error Occurred"); 
                exit(1)
            },
        }
        let _ = connection.close();
    }

    println!("Done");
    Some(())
}

pub async fn run_list_update(names: Option<Vec<String>>, id: bool) -> Option<()> {
    let (mut artists, connection) =
        ArtistDB::fetch_all().unwrap_or_else(|| panic!("Unable to fetch into the database"));
    if let Some(name) = names {
        artists.retain(|artist| {
            name.contains(if id {
                &artist.artist_spotify_id
            } else {
                &artist.artist_name
            })
        });
    }
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    let token = crate::spotify::Token::new(client_id.as_str(), client_secret.as_str())
        .await
        .unwrap();
    let spotify = Spotify::new(&token);
    println!();
    println!("------------------------------");
    println!("-------- New realease --------");
    println!("------------------------------");
    println!("");
    for artist in artists.iter_mut() {
        let updated = artist.update(&spotify, &connection).await;
        if let Some(updated) = updated {
            if updated {
                println!("\n");
                println!("{}", artist.default_format());
                (0..3)
                    .into_iter()
                    .for_each(|_| println!("------------------"));
            }
        } else {
            println!("An error occured for : {}", artist.artist_name)
        }
    }
    println!("Done");
    Some(())
}

pub fn run_list_show(
    all_info: bool,
    names: Option<Vec<String>>, id: bool
) -> Option<()> {
    let mut artists = ArtistDB::fetch_all()?.0;
    if let Some(name) = names {
        artists.retain(|artist| {
            name.contains(if id {
                &artist.artist_spotify_id
            } else {
                &artist.artist_name
            })
        });
    }

    for artist in artists {
        println!("\n");
        let artist_str = if all_info {
            artist.full_format()
        } else {
            artist.default_format()
        };
        println!("{}", artist_str);
        (0..3)
            .into_iter()
            .for_each(|_| println!("------------------"));
    }

    Some(())
}
// ------------------------- Init ------------------------- \\
pub fn run_init(client_id : String, client_secret : String) -> Option<()>{
    let mut app_dir = app_dir_pathbuf();
    app_dir.push(".env");
    let mut env = OpenOptions::new().create(true).truncate(true).write(true).open(app_dir).ok()?;
    env.write(format!("CLIENT_ID={}", client_id).as_bytes()).ok()?;
    env.write(format!("CLIENT_SECRET={}", client_secret).as_bytes()).ok()?;
    Some(())
}
// ------------------------ Search ------------------------ \\

pub async fn run_search(artist : bool, album : bool, track : bool,  market : Option<String>, limit : Option<u8>, offset : Option<u32>, item : String) -> Option<()> {
    let mut ressource_types = vec![];
    if artist { ressource_types.push(SpotifySearchType::Artist)}
    if track { ressource_types.push(SpotifySearchType::Track)}
    if album { ressource_types.push(SpotifySearchType::Album)}
    let spotify = Spotify::init().await;
    let result = spotify.search(item.as_str(), ressource_types, market, limit.map(|l| if l > 50 { 50 } else {l} ), offset, None).await?;
    result.iter().for_each(|(_,v)| {
        if v.items.is_empty() {
            println!("\n****   No Result   ****\n")
        }else {
            println!("{}", v.default_format());
        }
    });
    //println!("{:?}", result);

    Some(())
}