use std::process::exit;

use crate::{
    bindings::libcuesheetmaker::cue_file_format,
    spotify::{Spotify, SpotifySearchType},
    util,
};
use clap::{ArgGroup, Parser};

#[derive(Parser)]
/// Create the cue sheet by fechting the requiered information on the spotify api
#[clap(group(
    ArgGroup::new("names")
        .required(false)
        .multiple(true)
        .args(&["artist", "album"])
        .conflicts_with("album-id")
        .requires_all(&["artist", "album"])
)) ]
pub struct CueSheetFetch {
    /// Artist name
    #[clap(short, long)]
    artist: Option<String>,
    /// Album name
    #[clap(long, alias = "al")]
    album: Option<String>,
    /// Album spotify Id
    #[clap(long, alias = "id")]
    album_id: Option<String>,
    /// Output file
    #[clap(short, long, help = "Output file [stdout if not present]")]
    output: Option<String>,

    #[clap(long, alias = "cfn", default_value = "")]
    cue_file_name: String,

    #[clap(short, long, arg_enum)]
    format: cue_file_format,
    /// Display album total duration
    #[clap(long)]
    total_duration: bool,
    /// output path where fetched album illustration will be created
    #[clap(short, long)]
    image: Option<String>,
}

impl CueSheetFetch {
    pub async fn run(self) {
        let CueSheetFetch {
            artist,
            album,
            album_id,
            output,
            cue_file_name,
            format,
            total_duration,
            image,
        } = self;
        let spotify = Spotify::init().await;

        let album_id = if let Some(id) = album_id {
            id
        } else {
            let artist = artist.unwrap();
            let album = album.unwrap();
            match spotify
                .search(
                    format!("{} {}", album, artist).as_str(),
                    vec![SpotifySearchType::Album],
                    None,
                    Some(1),
                    None,
                    None,
                )
                .await
            {
                None => {
                    println!("No Resultat");
                    exit(1)
                }
                Some(result) => {
                    let data = result
                        .get(&crate::spotify::SpotifySearchKey::Albums)
                        .unwrap_or_else(|| {
                            println!("Unable to fetch the artist");
                            exit(1)
                        });
                    data.items.iter().filter_map(|ssri| match ssri {
                        crate::spotify::SpotifySearchResultItem::Album {
                            album_type: _,
                            artists: _,
                            external_urls: _,
                            available_markets: _,
                            href: _,
                            id,
                            images: _,
                            name: _,
                            release_date: _,
                            release_date_precision: _,
                            total_tracks: _,
                            a_type: _,
                            uri: _,
                        } => Some(id.clone()),
                        _ => None,
                    })
                }
                .collect::<Vec<String>>()
                .first()
                .unwrap_or_else(|| {
                    println!("Unable to get the artist Id");
                    exit(1)
                })
                .clone(),
            }
        };
        if let Err(e) = util::cuesheet_from_album_id(
            cue_file_name,
            format,
            output,
            album_id.as_str(),
            total_duration,
            image,
        )
        .await
        {
            println!("{}", e);
            exit(1)
        }
    }
}
