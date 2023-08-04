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

use std::{path::PathBuf, process::exit, str::FromStr};

use clap::{ArgEnum, Parser};
use tag_edit::{FlacTag, PictureFormat, ID3TAG};

#[derive(Clone, Copy, Debug, ArgEnum)]
pub enum FileType {
    Mp3,
    Flac,
}

/// Edit mp3 and flac file
#[derive(Parser)]
pub struct Edit {
    #[clap(long = "type", arg_enum)]
    file_type: FileType,
    /// Set the music title
    #[clap(short, long)]
    title: Option<String>,
    /// Set the track artist name
    #[clap(long)]
    artist: Option<String>,
    /// Set the album name
    #[clap(long)]
    album: Option<String>,
    /// Set the album artist
    #[clap(long)]
    artist_album: Option<String>,
    /// Set year
    #[clap(long)]
    year: Option<i16>,
    /// Set bpm
    #[clap(long)]
    bpm: Option<u16>,
    /// Set track position
    #[clap(long)]
    /// Set track position
    track_position: Option<u16>,
    /// Add images
    #[clap(long)]
    images: Option<Vec<String>>,
    /// Output the
    #[clap(short)]
    output: Option<String>,
    /// Audio file
    file: String,
}

impl Edit {
    pub fn run(self) {
        match &self.file_type {
            FileType::Mp3 => self.run_mp3(),
            FileType::Flac => self.run_flac(),
        }
    }

    fn run_mp3(self) {
        let Edit {
            file_type: _,
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
        } = self;
        let mut id3tag = match ID3TAG::from_path(file.as_str()) {
            Some(id3tag) => id3tag,
            None => {
                println!("File {} not found", &file);
                return;
            }
        };
        if let Some(title) = title {
            id3tag.set_title(title.as_str())
        }
        if let Some(artist) = artist {
            id3tag.set_artist(artist.as_str())
        }
        if let Some(album) = album {
            id3tag.set_album(album.as_str())
        }
        if let Some(artist_album) = artist_album {
            id3tag.set_album_artist(artist_album.as_str())
        }
        if let Some(year) = year {
            id3tag.set_year(year)
        }
        if let Some(bpm) = bpm {
            id3tag.set_bpm(bpm)
        }
        if let Some(tp) = track_position {
            id3tag.set_track_position(tp, None)
        }
        if let Some(images) = images {
            for image in images {
                let pathbuf_opt = PathBuf::from_str(&image.as_str()).ok();
                let extension_opt = pathbuf_opt.and_then(|pathbuf| {
                    pathbuf
                        .extension()
                        .map(|os| os.to_str().map(String::from))
                        .unwrap_or(Some("".into()))
                });
                let extension = match extension_opt {
                    Some(ext) => ext,
                    None => continue,
                };
                let _ = id3tag
                    .add_picture_from_file(
                        image.as_str(),
                        PictureFormat::OTHER(extension.to_string()),
                        None,
                        None,
                    )
                    .ok()
                    .unwrap_or_else(|| {
                        println!("Unable to add the picture");
                        exit(1)
                    });
            }
        }
        if let Some(output) = output {
            let _ = id3tag.write_tag(output.as_str()).unwrap_or_else(|_| {
                println!("Unable to write the file");
                exit(1)
            });
        } else {
            let _ = id3tag.overwrite_tag().unwrap_or_else(|_| {
                println!("Unable to write the file");
                exit(1)
            });
        }
    }

    fn run_flac(self) {
        let Edit {
            file_type: _,
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
        } = self;
        let mut flac = match FlacTag::from_path(file.as_str()) {
            Some(tag) => tag,
            None => {
                println!("Cannot open {}", &file);
                return;
            }
        };
        if let Some(title) = title {
            flac.set_title(title.as_str())
        }
        if let Some(artist) = artist {
            flac.set_artist(artist.as_str())
        }
        if let Some(album) = album {
            flac.set_album(album.as_str())
        }
        if let Some(artist_album) = artist_album {
            flac.set_album_artist(artist_album.as_str())
        }
        if let Some(year) = year {
            flac.set_date(year.to_string().as_str())
        }
        if let Some(bpm) = bpm {
            flac.set_bpm(bpm)
        }
        if let Some(tp) = track_position {
            flac.set_track_position(tp)
        }

        if let Some(pictures) = images {
            for image in pictures {
                let pathbuf_opt = PathBuf::from_str(&image.as_str()).ok();
                let extension_opt = pathbuf_opt.and_then(|pathbuf| {
                    pathbuf
                        .extension()
                        .map(|os| os.to_str().map(String::from))
                        .unwrap_or(Some("".into()))
                });
                let extension = match extension_opt {
                    Some(ext) => ext,
                    None => continue,
                };
                let _ = flac
                    .add_picture_from_path(
                        &image.as_str(),
                        tag_edit::PictureType::Other,
                        PictureFormat::OTHER(extension),
                        None,
                        400,
                        400,
                        16,
                        None,
                    )
                    .ok()
                    .unwrap_or_else(|| {
                        println!("Unable to add the picture");
                        exit(1)
                    });
            }
        }

        if let Some(output) = output {
            let _ = flac.write_flac(output.as_str()).unwrap_or_else(|_| {
                println!("Unable to write the file");
                exit(1)
            });
        } else {
            let _ = flac.overwrite_flac().unwrap_or_else(|_| {
                println!("Unable to write the file");
                exit(1)
            });
        }
    }
}
