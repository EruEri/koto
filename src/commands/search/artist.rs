use std::process::exit;

use clap::{ArgGroup, Parser};

use crate::{
    libs::spotify::{Spotify, SpotifyIncludeGroupe, SpotifySearchType},
    libs::util,
};

/// Search content related to an artist
#[derive(Debug, Parser)]
#[clap(group(
    ArgGroup::new("search_type")
    .required(false)
    .args(&["albums"])
    .conflicts_with("related_artists")
)) ]
pub struct Artist {
    #[clap(short, long)]
    /// Search artist's albums
    albums: bool,
    #[clap(long)]
    /// Search related artists
    related_artists: bool,
    #[clap(long)]
    /// Search by artist id
    id: bool,
    #[clap(short, long)]
    /// Display graohic result (cover, picture, etc ...)
    graphic: bool,
    #[clap(short, long, default_value_t = 3)]
    /// Result limit
    limit: u32,
    /// Output column
    #[clap(short, long, default_value_t = 1)]
    column: usize,
    /// Search item
    artist: String,
}

impl Artist {
    pub async fn run(self) {
        let Artist {
            albums,
            related_artists,
            id,
            graphic,
            limit,
            column,
            artist,
        } = self;
        let spotify = Spotify::init().await;
        let artist_id = if id {
            artist
        } else {
            let result = spotify
                .search(
                    artist.as_str(),
                    vec![SpotifySearchType::Artist],
                    None,
                    Some(1),
                    Some(0),
                    None,
                )
                .await
                .unwrap_or_else(|| {
                    println!("Wrong Api Response");
                    exit(1)
                });
            let items = result
                .get(&crate::libs::spotify::SpotifySearchKey::Artists)
                .unwrap_or_else(|| {
                    println!("Unable to get the artist");
                    exit(1)
                });
            let mut vec_artist_id = items
                .items
                .iter()
                .filter_map(|ssri| match ssri {
                    crate::libs::spotify::SpotifySearchResultItem::Artist {
                        external_urls: _,
                        followers: _,
                        genres: _,
                        href: _,
                        id,
                        images: _,
                        name: _,
                        popularity: _,
                        artist_type: _,
                        uri: _,
                    } => Some(id.clone()),
                    _ => None,
                })
                .collect::<Vec<String>>();
            if !vec_artist_id.is_empty() {
                vec_artist_id.remove(0)
            } else {
                println!("No artist returned");
                exit(1)
            }
        };
        match (albums, related_artists) {
            (true, true) => unreachable!("Albums and Related are mutualy exclued"),
            (true, false) => {
                let _albums = spotify
                    .artist_album(
                        artist_id,
                        vec![SpotifyIncludeGroupe::Album],
                        Some(limit),
                        None,
                        None,
                    )
                    .await
                    .unwrap_or_else(|| {
                        println!("Unable to fetch the related artist");
                        exit(1)
                    });
            }
            (false, true) => {
                let related_artists =
                    spotify
                        .related_artists(&artist_id)
                        .await
                        .unwrap_or_else(|| {
                            println!("Unable to fetch the related artist");
                            exit(1)
                        });
                let _ =
                    util::display_related_artist(&related_artists, column, limit as usize, graphic)
                        .await;
                return;
            }
            (false, false) => {
                let artist = spotify.artist(artist_id.as_str()).await.unwrap_or_else(|| {
                    println!("Unable to retrieve the artist");
                    exit(1)
                });
                println!("Name  : {}", artist.name);
                println!("Genre : {}", artist.genres.join("\n        "));
                if let Some(map) = artist.images.get(0) {
                    if let Some(url) = map.get("url") {
                        let url = url.as_str().unwrap();
                        let dyn_image = util::donwload_image(url).await;
                        if let Some(image) = dyn_image {
                            util::show_image(&image);
                        }
                    }
                }
            }
        }
    }
}
