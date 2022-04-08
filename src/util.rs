

use std::{future, cmp};

use image::DynamicImage;
use viuer::Config;

use crate::spotify::Artist;

pub(crate) async fn donwload_image(url : &str) -> Option<DynamicImage> {
    let image_bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;
    image::load_from_memory(&image_bytes).ok()
}

pub(crate) fn show_image(image : &DynamicImage) -> Option<()>{
    let config = Config {
        absolute_offset: false,
        x : 0,
        y : 0,
        width : Some(50),
        height : Some(50),
        ..Default::default()
    };
    viuer::print(image, &config).ok().map(|_| ())
}

pub (crate) fn show_image_config(image : &DynamicImage, config : &Config) -> Option<()>{
    viuer::print(image, config).ok().map(|_| ())
}

pub (crate) async fn display_related_artist(artists : &Vec<Artist>, chunck_size : usize) -> Option<()>{

    

    for chunk in artists.chunks(chunck_size) {
        'inner: for i in 0..chunck_size {
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

        for i in 0..chunck_size {
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

        for i in 0..chunck_size {
            let max_genre = chunk.iter()
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

        for i in 0..chunck_size {
            if let Some(artist) = chunk.get(i){
                artist.dynamic_image().await.map(|image| {
                    show_image_config(&image, &Config { absolute_offset: false, x: (i * 60) as u16, y: 0,restore_cursor : false ,width: Some(50), height: Some(50),..Default::default() });
                });
            }else {
                break ;
            }
        }
        
    }
    Some(())
}