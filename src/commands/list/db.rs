use std::{fs::OpenOptions, time::Duration};

use chrono::NaiveDate;

use crate::{
    config::{koto_base_dir, KOTO_DB_NAME},
    libs::spotify::{self, SpotifySearchResultItem, SpotifySearchType},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Artist {
    pub(crate) artist_name: String,
    pub(crate) artist_spotify_id: String,
    pub(crate) last_album: String,
    pub(crate) last_album_release_date: NaiveDate,
    pub(crate) last_album_spotify_id: String,
    pub(crate) last_album_url: String,
}

impl Artist {
    pub async fn from_id(spotify: &spotify::Spotify, id: &String) -> Option<Self> {
        let artist = spotify.artist(id).await?;
        let lastest_album = spotify.artist_lastest_album(artist.id.as_str()).await?;
        Some(Self {
            artist_name: artist.name,
            artist_spotify_id: artist.id,
            last_album: lastest_album.name,
            last_album_release_date: lastest_album.release_date,
            last_album_spotify_id: lastest_album.id,
            last_album_url: lastest_album.external_urls.get("spotify")?.clone(),
        })
    }

    pub async fn from_name(spotify: &spotify::Spotify, name: &String, id: bool) -> Option<Self> {
        let id = match id {
            true => name.clone(),
            false => {
                let map = spotify
                    .search(
                        name.as_str(),
                        vec![SpotifySearchType::Artist],
                        None,
                        Some(1),
                        Some(0),
                        None,
                    )
                    .await?;
                let artist = map.get(&crate::libs::spotify::SpotifySearchKey::Artists)?;
                let items = &artist.items;
                let artist_id = items.iter().find_map(|rssri| match rssri {
                    SpotifySearchResultItem::Artist { id, .. } => Some(id.clone()),
                    _ => None,
                })?;
                artist_id.clone()
            }
        };
        Self::from_id(spotify, &id).await
    }

    pub fn default_format(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("***   Artist Name   : {}   \n", self.artist_name).as_str());
        s.push_str(format!("***   Last Album    : {}   \n", self.last_album).as_str());
        s.push_str(
            format!(
                "***   Realease Date : {}   \n",
                self.last_album_release_date
            )
            .as_str(),
        );
        s
    }

    pub fn full_format(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("***   Artist Name   : {}   \n", self.artist_name).as_str());
        s.push_str(format!("***   Artist ID     : {}   \n", self.artist_spotify_id).as_str());
        s.push_str(format!("***   Last Album    : {}   \n", self.last_album).as_str());
        s.push_str(format!("***   Last album ID : {}   \n", self.last_album_spotify_id).as_str());
        s.push_str(
            format!(
                "***   Realease Date : {}   \n",
                self.last_album_release_date
            )
            .as_str(),
        );
        s.push_str(format!("***   Album Url     : {}   \n", self.last_album_url).as_str());
        s
    }

    pub async fn update(&mut self, spotify: &spotify::Spotify) -> bool {
        let lastest = match spotify
            .artist_lastest_album(self.artist_spotify_id.as_str())
            .await
        {
            Some(s) => s,
            None => return false,
        };
        let should_update = &lastest.release_date > &self.last_album_release_date;
        let () = match should_update {
            true => {
                self.last_album = lastest.name;
                self.last_album_spotify_id = lastest.id;
                self.last_album_release_date = lastest.release_date.clone();
                self.last_album_url = lastest.external_urls.get("spotify").unwrap().clone();
            }
            false => (),
        };
        should_update
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Artists {
    artists: Vec<Artist>,
}

impl Default for Artists {
    fn default() -> Self {
        Self {
            artists: Default::default(),
        }
    }
}

impl Artists {
    pub async fn update(&mut self, id: bool, artist: &Option<String>) {
        let spotify = spotify::Spotify::init().await;
        let mut artists = match artist {
            Some(name) => self
                .artists
                .iter_mut()
                .find(|elt| match id {
                    true => name == &elt.artist_spotify_id,
                    false => name == &elt.artist_name,
                })
                .map(|e| vec![e])
                .unwrap_or_default(),
            None => self.artists.iter_mut().collect(),
        };
        println!();
        println!("------------------------------");
        println!("-------- New realease --------");
        println!("------------------------------");
        println!("");
        let () = for artist in artists.iter_mut() {
            let () = tokio::time::sleep(Duration::from_millis(300)).await;
            match artist.update(&spotify).await {
                false => (),
                true => {
                    println!("\n");
                    println!("{}", artist.default_format());
                    (0..3)
                        .into_iter()
                        .for_each(|_| println!("------------------"));
                }
            }
        };
    }

    pub fn add(&mut self, artist: Artist) {
        self.artists.push(artist)
    }

    pub fn delete(&mut self, id: bool, name: &String) {
        self.artists.retain(|elt| match id {
            true => name != &elt.artist_spotify_id,
            false => name != &elt.artist_name,
        })
    }

    pub fn show(&self, full: bool) {
        self.artists.iter().for_each(|artist| match full {
            true => println!("{}", artist.default_format()),
            false => println!("{}", artist.full_format()),
        })
    }

    pub fn save(&self) {
        let koto_dir = koto_base_dir();
        let path = match koto_dir.find_data_file(KOTO_DB_NAME) {
            Some(some) => some,
            None => {
                println!("Error file not exist");
                return;
            }
        };
        let file = OpenOptions::new()
            .create(false)
            .truncate(true)
            .write(true)
            .read(false)
            .open(path)
            .unwrap();
        let _ = serde_json::to_writer(file, self);
        ()
    }

    pub fn deserialize() -> Option<Self> {
        let koto_dir = koto_base_dir();
        let path = koto_dir.find_data_file(KOTO_DB_NAME)?;
        let file = OpenOptions::new()
            .create(false)
            .truncate(false)
            .write(false)
            .read(true)
            .open(path)
            .unwrap();
        serde_json::from_reader(file).ok()
    }
}
