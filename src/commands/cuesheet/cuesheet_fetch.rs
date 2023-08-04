// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
// This file is part of Koto: A holdall music program                                         //
// Copyright (C) 2023 Yves Ndiaye                                                             //
//                                                                                            //
// Koto is free software: you can redistribute it and/or modify it under the terms            //
// of the GNU General Public License as published by the Free Software Foundation,            //
// either version 3 of the License, or (at your option) any later version.                    //
//                                                                                            //
// Koto is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;          //
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR           //
// PURPOSE.  See the GNU General Public License for more details.                             //
// You should have received a copy of the GNU General Public License along with Koto.         //
// If not, see <http://www.gnu.org/licenses/>.                                                //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

use std::process::exit;

use crate::{
    config::{check_credential_exist, extend_env},
    libs::spotify::{Spotify, SpotifySearchType},
    libs::util,
};
use clap::{ArgEnum, ArgGroup, Parser};
use cuesheet_rs::CueFileFormat;

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

    #[clap(long, alias = "cfn", default_value = "\"\"")]
    cue_file_name: String,

    #[clap(short, long, arg_enum)]
    format: CueFileFormatLocal,
    /// Display album total duration
    #[clap(long)]
    total_duration: bool,
    /// output path where fetched album illustration will be created
    #[clap(short, long)]
    image: Option<String>,
}

#[derive(Debug, Clone, Copy, ArgEnum)]
pub(crate) enum CueFileFormatLocal {
    BINARY,
    MOTOROLA,
    AIFF,
    WAVE,
    MP3,
}

impl CueFileFormatLocal {
    pub fn to_cuefileformat(&self) -> CueFileFormat {
        match self {
            CueFileFormatLocal::BINARY => CueFileFormat::BINARY,
            CueFileFormatLocal::MOTOROLA => CueFileFormat::MOTOROLA,
            CueFileFormatLocal::AIFF => CueFileFormat::AIFF,
            CueFileFormatLocal::WAVE => CueFileFormat::WAVE,
            CueFileFormatLocal::MP3 => CueFileFormat::MP3,
        }
    }
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
        let () = extend_env();
        let () = match check_credential_exist() {
            true => (),
            false => return,
        };
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
                        .get(&crate::libs::spotify::SpotifySearchKey::Albums)
                        .unwrap_or_else(|| {
                            println!("Unable to fetch the artist");
                            exit(1)
                        });
                    data.items.iter().filter_map(|ssri| match ssri {
                        crate::libs::spotify::SpotifySearchResultItem::Album {
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
            format.to_cuefileformat(),
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
