use std::cmp;

use chrono::Datelike;
use cuesheet_rs::{CueFileFormat, CueSheet, CueTrack, DurationFormat};
use image::DynamicImage;
use viuer::Config;

use super::spotify;

pub fn convert_mille_to_duration(milliemes: u64) -> DurationFormat {
    let secondes = milliemes / 1000;
    let milliemes = milliemes % 1000;
    let minutes = secondes / 60;
    let secondes = secondes % 60;
    DurationFormat::MinSecMil(minutes as u32, secondes as u32, milliemes as u32)
}

fn min_sec_mil_of_millies(milliemes: u64) -> (u32, u32, u32) {
    let secondes = milliemes / 1000;
    let milliemes = milliemes % 1000;
    let minutes = secondes / 60;
    let secondes = secondes % 60;
    (minutes as u32, secondes as u32, milliemes as u32)
}

pub(crate) async fn donwload_image(url: &str) -> Option<DynamicImage> {
    let image_bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;
    image::load_from_memory(&image_bytes).ok()
}

pub(crate) fn show_image(image: &DynamicImage) -> Option<()> {
    let config = Config {
        absolute_offset: false,
        x: 0,
        y: 0,
        width: Some(50), /*Some(50)*/
        height: None,    /*Some(50)*/
        ..Default::default()
    };
    viuer::print(&image, &config).ok().map(|_| ())
}

pub(crate) fn show_image_config(image: &DynamicImage, config: &Config) -> Option<()> {
    viuer::print(image, config).ok().map(|_| ())
}

pub(crate) async fn display_related_artist(
    artists: &Vec<super::spotify::Artist>,
    column: usize,
    limit: usize,
    graphic: bool,
) -> Option<()> {
    let artists = if artists.len() > limit {
        artists
            .into_iter()
            .enumerate()
            .filter_map(|(index, a)| if index < limit { Some(a) } else { None })
            .collect::<Vec<&super::spotify::Artist>>()
    } else {
        artists.iter().map(|a| a).collect()
    };

    for chunk in artists.chunks(column) {
        'inner: for i in 0..column {
            if let Some(artist) = chunk.get(i) {
                let name = &artist.name;
                let name_format = format!("Name   : {}", name);
                print!("{}", name_format);
                let space_len = if name_format.len() > 60 {
                    5
                } else {
                    60 - name_format.len()
                };
                (0..space_len).for_each(|_| print!(" "));
            } else {
                break 'inner;
            }
        }
        println!("");

        for i in 0..column {
            if let Some(artist) = chunk.get(i) {
                let id = &artist.id;
                let id_format = format!("Id     : {}", id);
                print!("{}", id_format);
                let space_len = if id_format.len() > 60 {
                    5
                } else {
                    60 - id_format.len()
                };
                (0..space_len).for_each(|_| print!(" "));
            } else {
                break;
            }
        }
        println!("");

        for i in 0..column {
            let _max_genre = chunk
                .iter()
                .map(|artist| artist.genres.len())
                .reduce(|x, y| cmp::max(x, y));
            if let Some(artist) = chunk.get(i) {
                let genres = &artist.genres;
                let genres_format = format!("Genres : {}", genres.join(", "));
                print!("{}", genres_format);
                let space_len = if genres_format.len() > 60 {
                    5
                } else {
                    60 - genres_format.len()
                };
                (0..space_len).for_each(|_| print!(" "));
            } else {
                break;
            }
        }
        println!("");
        if graphic {
            for i in 0..column {
                if let Some(artist) = chunk.get(i) {
                    artist.dynamic_image().await.map(|image| {
                        show_image_config(
                            &image,
                            &Config {
                                absolute_offset: false,
                                x: (i * 60) as u16,
                                y: 0,
                                restore_cursor: false,
                                width: Some(50),
                                height: None,
                                ..Default::default()
                            },
                        );
                    });
                } else {
                    break;
                }
            }
        }
    }
    Some(())
}

pub async fn cuesheet_from_album_id(
    filename: String,
    format: CueFileFormat,
    output: Option<String>,
    album_id: &str,
    total_duration: bool,
    image: Option<String>,
) -> Result<(), String> {
    // let mut argv = env::args().collect::<Vec<String>>();
    // let mut mapper_argv = argv.
    // iter_mut(
    // .map(|arg| (arg.as_mut_ptr() as * mut i8))
    // .collect::<Vec<*mut i8>>();
    // mapper_argv.push("\0".as_ptr() as *mut i8);

    let mut total = 0u64;
    let spotify = spotify::Spotify::init().await;
    let album = spotify
        .album(album_id.to_string())
        .await
        .ok_or("Unable to fetch the album".to_string())?;

    if let Some(image_path) = image {
        if let Some(map) = album.images.get(0) {
            if let Some(url) = map.get("url").and_then(|url| url.as_str()) {
                donwload_image(url).await.iter().for_each(|dyn_image| {
                    let _ = dyn_image.save(image_path.as_str());
                });
            }
        }
    }

    let mut cue_sheet = CueSheet::new(filename.as_str(), format);
    cue_sheet.add_title(album.name.as_str());

    if !album.artists.is_empty() {
        let album_artist = album
            .artists
            .iter()
            .map(|artist| artist.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        cue_sheet.add_performer(album_artist.as_str());
    }

    if let Some(Some(genres)) = album.genres {
        if !genres.is_empty() {
            let str_genres = genres.join(", ");

            let _ = cue_sheet.add_genre(&str_genres.as_str());
        }
    }

    let str_date = album.release_date.year().to_string();
    cue_sheet.add_rem("DATE", str_date.as_str());

    album.tracks.items.iter().for_each(|track| {
        let mut cuetrack =
            CueTrack::new(track.track_number as u32, cuesheet_rs::CueTrackMode::AUDIO);

        if !track.artists.is_empty() {
            let str_artist = track
                .artists
                .iter()
                .map(|artist| artist.name.to_owned())
                .collect::<Vec<String>>()
                .join(", ");
            let _ = cuetrack.add_performer(&str_artist.as_str());
        }
        total += track.duration_ms;
        let c_track_name = track.name.clone();
        cuetrack.add_index(1, convert_mille_to_duration(track.duration_ms));
        cuetrack.add_title(c_track_name.as_str());

        cue_sheet.add_track(cuetrack);
    });
    if total_duration {
        let (minutes, secondes, milliemes) = min_sec_mil_of_millies(total);
        println!(
            "ALBUM DURATION : {:#2}:{:#2}:{:#3}",
            minutes, secondes, milliemes
        );
    };

    let _ = match output {
        Some(output_path) => cue_sheet
            .export(true, output_path)
            .map_err(|e| format!("{}", e))?,
        None => println!("{}", cue_sheet.repr(true)),
    };

    Ok(())
}
