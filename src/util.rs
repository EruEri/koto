

use std::{cmp, fs::OpenOptions, io::Write};

use image::DynamicImage;
use viuer::Config;

use crate::{spotify::{Artist, self}, bindings::libcuesheetmaker::{duration, cue_file_format, cue_sheet, cue_track, caml_wrapper_starup}, sql::Date};

pub fn convert_mille_to_duration(milliemes: u64) -> duration {
    let secondes = milliemes / 1000;
    let milliemes = milliemes % 1000;
    let minutes = secondes / 60;
    let secondes = secondes % 60;
    duration::minuts_seconde_milliemes_format(minutes as i32, secondes as i32, milliemes as i32)
}

fn min_sec_mil_of_millies(milliemes: u64) -> (u32, u32, u32) {
    let secondes = milliemes / 1000;
    let milliemes = milliemes % 1000;
    let minutes = secondes / 60;
    let secondes = secondes % 60;
    (minutes as u32, minutes as u32, milliemes as u32)
}

pub(crate) async fn donwload_image(url : &str) -> Option<DynamicImage> {
    let image_bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;
    image::load_from_memory(&image_bytes).ok()
}

pub(crate) fn show_image(image : &DynamicImage) -> Option<()>{
    let config = Config {
        absolute_offset: false,
        x : 0,
        y : 0,
        width : Some(50) /*Some(50)*/,
        height : None /*Some(50)*/,
        ..Default::default()
    };
    viuer::print(&image, &config).ok().map(|_| ())
}

pub (crate) fn show_image_config(image : &DynamicImage, config : &Config) -> Option<()>{
    viuer::print(image, config).ok().map(|_| ())
}

pub (crate) async fn display_related_artist(artists : &Vec<Artist>, column : usize, limit : usize, graphic : bool) -> Option<()>{

    let artists = if artists.len() > limit { 
        artists.into_iter()
        .enumerate()
        .filter_map(|(index, a)| if index < limit { Some( a ) } else { None } )
        .collect::<Vec<&Artist>>()
    } else { 
            artists.iter()
            .map(|a| a)
            .collect()
    };

    for chunk in artists.chunks(column) {
        'inner: for i in 0..column {
            if let Some(artist) = chunk.get(i){
                let name = &artist.name;
                let name_format = format!("Name   : {}", name);
                print!("{}", name_format);
                let space_len = if name_format.len() > 60 {5} else { 60 - name_format.len()};
                (0..space_len).for_each(|_| print!(" "));
            }else {
                break 'inner;
            }
        }
        println!("");

        for i in 0..column {
            if let Some(artist) = chunk.get(i){
                let id = &artist.id;
                let id_format = format!("Id     : {}", id);
                print!("{}", id_format);
                let space_len = if id_format.len() > 60 {5} else { 60 - id_format.len()};
                (0..space_len).for_each(|_| print!(" "));
            }else {
                break ;
            }
        }
        println!("");

        for i in 0..column {
            let _max_genre = chunk.iter()
            .map(|artist| artist.genres.len())
            .reduce(|x, y| cmp::max(x, y));
            if let Some(artist) = chunk.get(i){
                let genres = &artist.genres;
                let genres_format = format!("Genres : {}", genres.join(", "));
                print!("{}", genres_format);
                let space_len = if genres_format.len() > 60 {5} else { 60 - genres_format.len()};
                (0..space_len).for_each(|_| print!(" "));
            }else {
                break ;
            }
        }
        println!("");
        if graphic {
            for i in 0..column {
                if let Some(artist) = chunk.get(i){
                    artist.dynamic_image().await.map(|image| {
                        show_image_config(&image, &Config { absolute_offset: false, x: (i * 60) as u16, y: 0,restore_cursor : false ,width: Some(50), height: None,..Default::default() });
                    });
                }else {
                    break ;
                }
            }
        }
    }
    Some(())
}

pub async fn cuesheet_from_album_id(mut filename: String, format: cue_file_format, output: Option<String>, album_id: &str, total_duration: bool) -> Result<(), String> {
    // let mut argv = env::args().collect::<Vec<String>>();
    // let mut mapper_argv = argv.
    // iter_mut()
    // .map(|arg| {arg.push('\0'); arg})
    // .map(|arg| (arg.as_mut_ptr() as * mut i8))
    // .collect::<Vec<*mut i8>>();
    // mapper_argv.push("\0".as_ptr() as *mut i8);
    unsafe { 
        caml_wrapper_starup();
    }

    let mut total = 0u64;
    
    filename.push('\0');
    let spotify = spotify::Spotify::init().await;
    let album = spotify.album(album_id.to_string()).await.ok_or( "Unable to fetch the album".to_string() )?;
    
    let mut cue_sheet = cue_sheet::new_empty_sheet(filename.as_str(), format);
    cue_sheet.add_title(album.name.as_str());

    
    if let Some(Some(genres)) = album.genres {
        if !genres.is_empty() {
            let mut str_genres = genres.join(", ");
            str_genres.push('\0');
            let _ = cue_sheet.add_genre(&str_genres.as_str());
        }
    }
    if let Some(date) = Date::from_str(&album.release_date) {
        let mut str_date = date.year.to_string();
        str_date.push('\0');
        cue_sheet.add_rem("DATE\0", str_date.as_str());
    }
    album.tracks.items.iter().for_each(|track| {
        let mut cuetrack = cue_track::new_empty_track(track.track_number as i32, crate::bindings::libcuesheetmaker::cue_track_mode::AUDIO);

        if !track.artists.is_empty() {
            let mut str_artist = track.artists.iter().map(|artist| artist.name.to_owned() ).collect::<Vec<String>>().join(", ");
            str_artist.push('\0');
            let _ = cuetrack.add_performer(&str_artist.as_str());
        }
        total += track.duration_ms;
        let mut c_track_name = track.name.clone();
        c_track_name.push('\0');
        cuetrack.add_index(1, convert_mille_to_duration(track.duration_ms));
        cuetrack.add_title(c_track_name.as_str());

        cue_sheet.add_track(&cuetrack);
    });
    if total_duration {
        let (minutes, secondes, milliemes) = min_sec_mil_of_millies(total);
        println!("ALBUM DURATION : {:#2}:{:#2}:{:#3}", minutes, secondes, milliemes);
    }
    let str_cuesheet = cue_sheet.to_format(true).ok_or("Cannot generate the string representation of the sheet")?;
    if let Some(output_path) = output {
        let _ = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_path)
        .map_err(|_| "Cannot open the file")?
        .write_all(str_cuesheet.as_bytes())
        .map_err(|_| "Error while writing to the file");
    } else {
        
        println!("{}", str_cuesheet)
    }

    Ok(())
}