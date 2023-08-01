#![allow(unused)]

use chrono::NaiveDate;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, process::exit};
use viuer::resize;

use base64::encode;
use reqwest::{RequestBuilder, StatusCode};
use serde_json::Value;

use crate::libs::util;

#[derive(Debug, Clone)]
pub struct Token {
    access_token: String,
    token_type: String,
    expire_in: u32,
}

impl Token {
    pub async fn new(client_id: &str, client_secret: &str) -> Option<Self> {
        let creditential = format!("{}:{}", client_id, client_secret);
        let encoded = format!("Basic {}", encode(creditential));
        let request = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .header("Authorization", encoded)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .await;
        if let Ok(r) = request {
            if r.status() != StatusCode::OK {
                None
            } else {
                let value = r.json::<Value>().await.ok()?;
                let access_token = value.as_object()?["access_token"].as_str()?.to_string();
                let token_type = value.as_object()?["token_type"].as_str()?.to_string();
                let expire_in: u32 = value.as_object()?["expires_in"].as_u64()? as u32;
                Some(Self {
                    access_token,
                    token_type,
                    expire_in,
                })
            }
        } else {
            None
        }
    }
}

pub struct Spotify {
    token: Token,
}

impl Spotify {
    pub fn new(token: &Token) -> Self {
        Self {
            token: token.clone(),
        }
    }
    pub async fn init() -> Self {
        let client_id = std::env::var("CLIENT_ID").unwrap();
        let client_secret = std::env::var("CLIENT_SECRET").unwrap();
        let token = Token::new(client_id.as_str(), client_secret.as_str())
            .await
            .unwrap_or_else(|| panic!("Unable to connect to the api"));
        Self::new(&token)
    }
    fn search_end_point() -> String {
        "https://api.spotify.com/v1/search".into()
    }
    fn track_end_point() -> String {
        "https://api.spotify.com/v1/tracks/".into()
    }
    fn album_end_point() -> String {
        "https://api.spotify.com/v1/albums/".into()
    }

    fn artist_end_point() -> String {
        "https://api.spotify.com/v1/artists/".into()
    }
    fn audio_feature_end_point() -> String {
        "https://api.spotify.com/v1/audio-features/".into()
    }
    fn audio_analysis_end_point() -> String {
        "https://api.spotify.com/v1/audio-analysis/".into()
    }

    fn create_url(
        end_point: &String,
        r_type: &SpotifyRessourceType,
        spotify_ids: Vec<String>,
        market: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
        included_genre: Vec<SpotifyIncludeGroupe>,
    ) -> RequestBuilder {
        let ids = if spotify_ids.len() > 1 {
            let s = String::from("?ids=") + &spotify_ids.join(",").to_string();
            s
        } else {
            spotify_ids.first().unwrap().clone()
        };
        let base_url = format!("{}{}", end_point, ids);
        let base_url = match r_type {
            SpotifyRessourceType::ArtistAlbum => format!("{}{}", base_url, "/albums"),
            SpotifyRessourceType::ArtistTopTrack => format!("{}{}", base_url, "/top-tracks"),
            SpotifyRessourceType::AlbumTrack => format!("{}{}", base_url, "/tracks"),
            SpotifyRessourceType::RelatedArtist => format!("{}{}", base_url, "/related-artists"),
            _ => base_url,
        };
        let req = reqwest::Client::new();
        let mut builder = req.get(base_url);
        if let Some(market) = market {
            builder = builder.query(&[("market", market)]);
        }
        if let Some(offset) = offset {
            builder = builder.query(&[("offset", offset)]);
        }
        if let Some(limit) = limit {
            builder = builder.query(&[("limit", limit)])
        }

        if !included_genre.is_empty() {
            let genre = included_genre
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
            builder = builder.query(&[("include_groups", genre)]);
        }
        builder
    }

    fn setup_url_request(
        &self,
        r_type: &SpotifyRessourceType,
        spotify_ids: Vec<String>,
        market: Option<String>,
        limit: Option<u32>,
        offset: Option<u32>,
        included_genre: Vec<SpotifyIncludeGroupe>,
    ) -> RequestBuilder {
        let end_point = &match r_type {
            &SpotifyRessourceType::Album | &SpotifyRessourceType::AlbumTrack => {
                Self::album_end_point()
            }
            &SpotifyRessourceType::Artist
            | &SpotifyRessourceType::ArtistAlbum
            | &SpotifyRessourceType::ArtistTopTrack
            | &SpotifyRessourceType::RelatedArtist => Self::artist_end_point(),
            &SpotifyRessourceType::Track => Self::track_end_point(),
            &SpotifyRessourceType::AudioFeature => Self::audio_feature_end_point(),
            &SpotifyRessourceType::AudioAnalysis => Self::audio_analysis_end_point(),
        };
        let rb = Self::create_url(
            end_point,
            r_type,
            spotify_ids,
            market,
            limit,
            offset,
            included_genre,
        );
        let rb = rb
            .header(
                "Authorization",
                format!("Bearer {}", self.token.access_token),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");
        rb
    }
    fn setup_search_request(
        &self,
        query: &str,
        item_type: Vec<SpotifySearchType>,
        market: Option<String>,
        limit: Option<u8>,
        offset: Option<u32>,
        include_external: Option<bool>,
    ) -> RequestBuilder {
        let req = reqwest::Client::new();
        let mut builder = req.get(Self::search_end_point());
        let item_type = item_type
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",");
        builder = builder.query(&[("q", query)]).query(&[("type", item_type)]);
        if let Some(market) = market {
            builder = builder.query(&[("market", market)]);
        }
        if let Some(offset) = offset {
            builder = builder.query(&[("offset", offset)]);
        }
        if let Some(limit) = limit {
            builder = builder.query(&[("limit", limit)]);
        }
        if include_external.unwrap_or(false) {
            builder = builder.query(&[("include_external", "audio")])
        }
        builder = builder
            .header(
                "Authorization",
                format!("Bearer {}", self.token.access_token),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");
        builder
    }
    pub async fn search(
        &self,
        query: &str,
        item_type: Vec<SpotifySearchType>,
        market: Option<String>,
        limit: Option<u8>,
        offset: Option<u32>,
        include_external: Option<bool>,
    ) -> Option<HashMap<SpotifySearchKey, SpotifySearchResult>> {
        // let response = self.setup_search_request(query, item_type, market, limit, offset, include_external).send().await;
        // match response {
        //     Err(e) => {println!("Errror search : {:?}", e); None},
        //     Ok(re) => {
        //         println!("{:?}", &re.json::<Value>().await);
        //         unreachable!("End");
        //         re.json::<HashMap<SpotifySearchKey, SpotifySearchResult>>()
        //         .await
        //         .ok()
        //     },
        // }
        self.setup_search_request(query, item_type, market, limit, offset, include_external)
            .send()
            .await
            .ok()?
            .json::<HashMap<SpotifySearchKey, SpotifySearchResult>>()
            .await
            .ok()
    }
    pub async fn artist(&self, artist_id: &str) -> Option<Artist> {
        let rb = self.setup_url_request(
            &SpotifyRessourceType::Artist,
            vec![artist_id.into()],
            None,
            None,
            None,
            vec![],
        );
        if let Ok(response) = rb.send().await {
            response.json::<Artist>().await.ok()
        } else {
            None
        }
    }

    pub async fn album(&self, album_id: String) -> Option<Album> {
        let rb = self.setup_url_request(
            &SpotifyRessourceType::Album,
            vec![album_id],
            None,
            None,
            None,
            vec![],
        );
        rb.send().await.ok()?.json().await.ok()
    }

    pub async fn _artists(&self, artist_ids: Vec<String>) -> Option<Vec<Value>> {
        let rb = self.setup_url_request(
            &SpotifyRessourceType::Artist,
            artist_ids,
            None,
            None,
            None,
            vec![],
        );
        if let Ok(response) = rb.send().await {
            response.json::<Vec<Value>>().await.ok()
        } else {
            None
        }
    }
    pub async fn album_tracks(&self, album_id: String) -> Option<SpotifyAlbumTrackResult> {
        let rb = self.setup_url_request(
            &SpotifyRessourceType::AlbumTrack,
            vec![album_id],
            None,
            Some(50),
            None,
            vec![],
        );
        rb.send().await.ok()?.json().await.ok()
    }
    pub async fn artist_album(
        &self,
        artist_id: String,
        included_genre: Vec<SpotifyIncludeGroupe>,
        limit: Option<u32>,
        market: Option<String>,
        offset: Option<u32>,
    ) -> Option<ArtistAlbum> {
        let rb = self.setup_url_request(
            &SpotifyRessourceType::ArtistAlbum,
            vec![artist_id],
            market,
            limit,
            offset,
            included_genre,
        );
        if let Ok(response) = rb.send().await {
            response.json::<ArtistAlbum>().await.ok()
        } else {
            None
        }
    }
    pub async fn related_artists(&self, artist_id: &String) -> Option<Vec<Artist>> {
        let rb = self.setup_url_request(
            &SpotifyRessourceType::RelatedArtist,
            vec![artist_id.clone()],
            None,
            None,
            None,
            vec![],
        );
        if let Ok(response) = rb.send().await {
            let map = response
                .json::<HashMap<SpotifySearchKey, Vec<Artist>>>()
                .await
                .ok()?;
            map.into_iter().map(|(k, v)| v).find(|_| true)
        } else {
            None
        }
    }
}

impl Spotify {
    pub async fn artist_lastest_album(&self, artist_id: &str) -> Option<AlbumItems> {
        let mut album = self
            .artist_album(
                artist_id.into(),
                vec![SpotifyIncludeGroupe::Album, SpotifyIncludeGroupe::Single],
                Some(40),
                None,
                Some(0),
            )
            .await?;
        album
            .items
            .sort_by(|a, b| a.release_date.partial_cmp(&b.release_date).unwrap());
        let items = album.items.remove(0);
        Some(items)
    }

    pub async fn get_artist_id(&self, artist_name: String) -> Option<String> {
        let result = self
            .search(
                artist_name.as_str(),
                vec![SpotifySearchType::Artist],
                None,
                Some(1),
                Some(0),
                None,
            )
            .await?;
        let items = result.get(&crate::libs::spotify::SpotifySearchKey::Artists)?;
        let mut vec_artist_id = items
            .items
            .iter()
            .filter_map(|ssri| match ssri {
                crate::libs::spotify::SpotifySearchResultItem::Artist {
                    external_urls,
                    followers,
                    genres,
                    href,
                    id,
                    images,
                    name,
                    popularity,
                    artist_type,
                    uri,
                } => Some(id.clone()),
                _ => None,
            })
            .collect::<Vec<String>>();
        if !vec_artist_id.is_empty() {
            Some(vec_artist_id.remove(0))
        } else {
            None
        }
    }
}

pub enum SpotifyRessourceType {
    Track,
    Artist,
    ArtistAlbum,
    ArtistTopTrack,
    Album,
    AlbumTrack,
    AudioFeature,
    RelatedArtist,
    AudioAnalysis,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpotifyIncludeGroupe {
    Single,
    Album,
    AppearsOn,
    Compilation,
}

impl Display for SpotifyIncludeGroupe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SpotifyIncludeGroupe::Single => "single",
            SpotifyIncludeGroupe::Album => "album",
            SpotifyIncludeGroupe::AppearsOn => "appears_on",
            SpotifyIncludeGroupe::Compilation => "compilation",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum SpotifySearchType {
    Album,
    Artist,
    Track,
}

impl Display for SpotifySearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SpotifySearchType::Album => "album",
            SpotifySearchType::Artist => "artist",
            SpotifySearchType::Track => "track",
        };
        write!(f, "{}", s)
    }
}
#[derive(Debug, Deserialize)]
pub struct Copyrights {
    text: String,
    #[serde(rename = "type")]
    c_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub(crate) album_type: String,
    pub(crate) artists: Vec<SpotifySearchAlbumArtist>,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) available_markets: Option<Option<Vec<String>>>,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) copyrights: Option<Option<Vec<Copyrights>>>,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) external_ids: Option<Option<HashMap<String, String>>>,
    pub(crate) external_urls: HashMap<String, String>,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) genres: Option<Option<Vec<String>>>,
    pub(crate) href: String,
    pub(crate) id: String,
    pub(crate) images: Vec<HashMap<String, Value>>,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) label: Option<Option<String>>,
    pub(crate) name: String,
    pub(crate) popularity: Option<Option<i32>>,
    pub(crate) release_date: NaiveDate,
    pub(crate) release_date_precision: String,
    pub(crate) total_tracks: u32,
    pub(crate) tracks: SpotifyAlbumTrackResult,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) restrictions: Option<Option<HashMap<String, Value>>>,
    #[serde(rename = "type")]
    pub(crate) a_type: String,
    pub(crate) uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub(crate) external_urls: HashMap<String, String>,
    pub(crate) followers: HashMap<String, Value>,
    pub(crate) genres: Vec<String>,
    pub(crate) href: String,
    pub(crate) id: String,
    pub(crate) images: Vec<HashMap<String, Value>>,
    pub(crate) name: String,
    pub(crate) popularity: i32,
    #[serde(rename = "type")]
    pub(crate) artist_type: String,
    pub(crate) uri: String,
}

impl Artist {
    pub(crate) async fn dynamic_image(&self) -> Option<DynamicImage> {
        let url = self.images.get(0)?.get("url")?.as_str()?;
        util::donwload_image(url).await
    }

    pub(crate) async fn dynamic_image_resize(
        &self,
        nwidth: Option<u32>,
        nheight: Option<u32>,
    ) -> Option<DynamicImage> {
        self.dynamic_image()
            .await
            .map(|image| resize(&image, nwidth, nheight))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistAlbum {
    pub(crate) href: String,
    pub(crate) items: Vec<AlbumItems>,
    pub(crate) limit: u8,
    pub(crate) next: Option<String>,
    pub(crate) offset: u32,
    pub(crate) previous: Option<String>,
    pub(crate) total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumItems {
    pub(crate) album_group: String,
    pub(crate) album_type: String,
    pub(crate) artists: Vec<Value>,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) available_markets: Option<Option<Vec<String>>>,
    pub(crate) external_urls: HashMap<String, String>,
    pub(crate) href: String,
    pub(crate) id: String,
    pub(crate) images: Vec<HashMap<String, Value>>,
    pub(crate) name: String,
    pub(crate) release_date: NaiveDate,
    pub(crate) release_date_precision: String,
    pub(crate) total_tracks: u8,
    #[serde(rename = "type")]
    pub(crate) r#type: String,
    pub(crate) uri: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String")]
pub enum SpotifySearchKey {
    Tracks,
    Artists,
    Albums,
    Playlists,
    Shows,
    Episodes,
}

pub struct SpotifyKeyError;

impl Display for SpotifyKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spotify Error Key")
    }
}

impl TryFrom<String> for SpotifySearchKey {
    type Error = SpotifyKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "tracks" => Ok(Self::Tracks),
            "artists" => Ok(Self::Artists),
            "albums" => Ok(Self::Albums),
            "playlists" => Ok(Self::Playlists),
            "shows" => Ok(Self::Shows),
            "episodes" => Ok(Self::Episodes),
            _ => Err(SpotifyKeyError {}),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SpotifySearchResult {
    pub(crate) href: String,
    pub(crate) items: Vec<SpotifySearchResultItem>,
    pub(crate) limit: u8,
    pub(crate) next: Option<String>,
    pub(crate) offset: u32,
    pub(crate) previous: Option<String>,
    pub(crate) total: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyAlbumTrackResult {
    pub(crate) href: String,
    pub(crate) items: Vec<TrackAlbum>,
    pub(crate) limit: u8,
    pub(crate) next: Option<String>,
    pub(crate) offset: u32,
    pub(crate) previous: Option<String>,
    pub(crate) total: u32,
}
#[derive(Debug, Deserialize)]
pub struct TrackAlbum {
    pub(crate) artists: Vec<SpotifySearchAlbumArtist>,
    #[serde(default, with = "serde_with::rust::double_option")]
    pub(crate) available_markets: Option<Option<Vec<String>>>,
    pub(crate) disc_number: u16,
    pub(crate) duration_ms: u64,
    pub(crate) explicit: bool,
    pub(crate) external_urls: HashMap<String, String>,
    pub(crate) href: String,
    pub(crate) id: String,
    pub(crate) is_local: bool,
    pub(crate) name: String,
    pub(crate) preview_url: String,
    pub(crate) track_number: u16,
    #[serde(rename = "type")]
    pub(crate) _type: String,
    pub(crate) uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SpotifySearchResultItem {
    Track {
        album: SpotifySearchTrackAlbum,
        artists: Vec<SpotifySearchAlbumArtist>,
        #[serde(default, with = "serde_with::rust::double_option")]
        available_markets: Option<Option<Vec<String>>>,
        disc_number: u16,
        duration_ms: u64,
        explicit: bool,
        external_ids: HashMap<String, String>,
        external_urls: HashMap<String, String>,
        href: String,
        id: String,
        is_local: bool,
        name: String,
        popularity: u32,
        preview_url: Value,
        track_number: u16,
        #[serde(rename = "type")]
        t_type: String,
        uri: String,
    },
    Artist {
        external_urls: HashMap<String, String>,
        followers: HashMap<String, Value>,
        genres: Vec<String>,
        href: String,
        id: String,
        images: Vec<HashMap<String, Value>>,
        name: String,
        popularity: i32,
        #[serde(rename = "type")]
        artist_type: String,
        uri: String,
    },
    Album {
        album_type: String,
        artists: Vec<SpotifySearchAlbumArtist>,
        external_urls: HashMap<String, String>,
        #[serde(default, with = "serde_with::rust::double_option")]
        available_markets: Option<Option<Vec<String>>>,
        href: String,
        id: String,
        images: Vec<HashMap<String, Value>>,
        name: String,
        release_date: String,
        release_date_precision: String,
        total_tracks: u32,
        #[serde(rename = "type")]
        a_type: String,
        uri: String,
    },
}
#[derive(Debug, Deserialize)]
pub struct SpotifySearchTrackAlbum {
    album_type: String,
    artists: Vec<SpotifySearchAlbumArtist>,
    #[serde(default, with = "serde_with::rust::double_option")]
    available_markets: Option<Option<Vec<String>>>,
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    images: Vec<HashMap<String, Value>>,
    name: String,
    release_date: String,
    release_date_precision: String,
    total_tracks: u32,
    #[serde(rename = "type")]
    a_type: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotifySearchAlbumArtist {
    pub(crate) external_urls: HashMap<String, String>,
    pub(crate) href: String,
    pub(crate) id: String,
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) r_type: String,
    pub(crate) uri: String,
}

impl SpotifySearchResultItem {
    pub fn default_format(&self) -> String {
        let mut s = String::new();
        match self {
            SpotifySearchResultItem::Track {
                album,
                artists,
                available_markets: _,
                disc_number: _,
                duration_ms: _,
                explicit: _,
                external_ids: _,
                external_urls: _,
                href: _,
                id,
                is_local: _,
                name,
                popularity: _,
                preview_url: _,
                track_number: _,
                t_type: _,
                uri: _,
            } => {
                s.push_str(format!("****   Song Name   : {}\n", name).as_str());
                s.push_str(format!("****   Song ID     : {}\n", id).as_str());
                let mut couples = artists
                    .iter()
                    .map(|artist| (artist.name.clone(), artist.id.clone()))
                    .collect::<Vec<_>>();
                let couple_total = couples.len();
                let (name, id) = couples.remove(0);
                s.push_str(
                    format!(
                        "****   Artist{}      : {}  := ID : {}\n",
                        if couple_total > 1 { "s" } else { "" },
                        name,
                        id
                    )
                    .as_str(),
                );
                couples.iter().for_each(|(s_name, s_id)| {
                    s.push_str(
                        format!("                     : {}  := ID : {}\n", s_name, s_id).as_str(),
                    )
                });
                s.push_str(format!("****   Album       : {}\n", album.name.clone()).as_str());
            }
            SpotifySearchResultItem::Artist {
                external_urls: _,
                followers: _,
                genres,
                href: _,
                id,
                images: _,
                name,
                popularity: _,
                artist_type: _,
                uri: _,
            } => {
                s.push_str(format!("****   Artist Name   : {}\n", name).as_str());
                s.push_str(format!("****   Artist ID     : {}\n", id).as_str());
                let genres = genres.join("\n                     : ");
                s.push_str(format!("****   Genre         : {}\n", genres).as_str());
            }
            SpotifySearchResultItem::Album {
                album_type,
                artists,
                external_urls: _,
                available_markets: _,
                href: _,
                id,
                images: _,
                name,
                release_date,
                release_date_precision: _,
                total_tracks,
                a_type: _,
                uri: _,
            } => {
                s.push_str(format!("****   Album Name    : {}\n", name).as_str());
                s.push_str(format!("****   Album ID      : {}\n", id).as_str());
                let mut couples = artists
                    .iter()
                    .map(|artist| (artist.name.clone(), artist.id.clone()))
                    .collect::<Vec<_>>();
                let couple_total = couples.len();
                let (name, id) = couples.remove(0);
                s.push_str(
                    format!(
                        "****   Artist{}       {}: {}  := ID : {}\n",
                        if couple_total > 1 { "s" } else { "" },
                        if couple_total > 1 { "" } else { " " },
                        name,
                        id
                    )
                    .as_str(),
                );
                s.push_str(format!("****   Release Date  : {}\n", release_date).as_str());
                s.push_str(format!("****   Total Tracks  : {}\n", total_tracks).as_str());
                s.push_str(format!("****   Album Type    : {}\n", album_type).as_str());
            }
        }
        s
    }

    pub async fn show_result(&self, graphic: bool) -> Option<()> {
        // print!("\n\n");
        match self {
            SpotifySearchResultItem::Track {
                album,
                artists,
                id,
                name,
                ..
            } => {
                print!("****   Song Name   : {}\n", name);
                print!("****   Song ID     : {}\n", id);
                let mut couples = artists
                    .iter()
                    .map(|artist| (artist.name.clone(), artist.id.clone()))
                    .collect::<Vec<_>>();
                let couple_total = couples.len();
                let (name, id) = couples.remove(0);
                print!(
                    "****   Artist{}      : {}  := ID : {}\n",
                    if couple_total > 1 { "s" } else { "" },
                    name,
                    id
                );
                couples.iter().for_each(|(s_name, s_id)| {
                    print!("                     : {}  := ID : {}\n", s_name, s_id)
                });
                print!("****   Album       : {}\n", album.name.clone());
                if graphic {
                    let url_album = album.images.get(0)?.get("url")?.as_str()?;
                    util::show_image(&util::donwload_image(url_album).await?);
                    Some(println!("\n\n"))
                } else {
                    Some(())
                }
            }
            SpotifySearchResultItem::Artist {
                external_urls: _,
                followers: _,
                genres,
                href: _,
                id,
                images,
                name,
                popularity: _,
                artist_type: _,
                uri: _,
            } => {
                print!("****   Artist Name   : {}\n", name);
                print!("****   Artist ID     : {}\n", id);
                let genres = genres.join("\n                     : ");
                print!("****   Genre         : {}\n", genres);
                if graphic {
                    let url = images.get(0)?.get("url")?.as_str()?;
                    util::show_image(&util::donwload_image(url).await?);
                    Some(println!("\n\n"))
                } else {
                    Some(())
                }
            }
            SpotifySearchResultItem::Album {
                album_type,
                artists,
                external_urls: _,
                available_markets: _,
                href: _,
                id,
                images,
                name,
                release_date,
                release_date_precision: _,
                total_tracks,
                a_type: _,
                uri: _,
            } => {
                print!("****   Album Name    : {}\n", name);
                print!("****   Album ID      : {}\n", id);
                let mut couples = artists
                    .iter()
                    .map(|artist| (artist.name.clone(), artist.id.clone()))
                    .collect::<Vec<_>>();
                let couple_total = couples.len();
                let (name, id) = couples.remove(0);
                print!(
                    "****   Artist{}       {}: {}  := ID : {}\n",
                    if couple_total > 1 { "s" } else { "" },
                    if couple_total > 1 { "" } else { " " },
                    name,
                    id
                );
                print!("****   Release Date  : {}\n", release_date);
                print!("****   Total Tracks  : {}\n", total_tracks);
                print!("****   Album Type    : {}\n", album_type);
                if graphic {
                    let url = images.get(0)?.get("url")?.as_str()?;
                    util::show_image(&util::donwload_image(url).await?);
                    Some(println!("\n\n"))
                } else {
                    Some(())
                }
            }
        }
    }
}

impl SpotifySearchResult {
    pub fn default_format(&self) -> String {
        let mut s = String::new();

        s.push('\n');
        s.push('\n');
        let mut section_track = true;
        let mut section_artist = true;
        let mut section_album = true;
        let mut items_count = self.offset;
        self.items.iter().for_each(|i| {
            match i {
                SpotifySearchResultItem::Track { .. } => {
                    if section_track {
                        s.push_str("----------------------------------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push_str("---------------  Tracks  ---------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push('\n');
                        s.push('\n');
                        s.push_str(
                            format!("------ Number of items :  {}   -------\n", self.total)
                                .as_str(),
                        );
                        s.push_str(
                            format!("------ Result limit    :  {}   -------\n", self.limit)
                                .as_str(),
                        );
                        s.push_str(
                            format!("------ Result offset   :  {}   -------\n", self.offset)
                                .as_str(),
                        );
                        s.push('\n');
                        section_track = false;
                        section_artist = true;
                        section_album = true;
                        items_count = self.offset
                    }
                }
                SpotifySearchResultItem::Artist { .. } => {
                    if section_artist {
                        s.push_str("----------------------------------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push_str("---------------- Artists ---------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push('\n');
                        s.push('\n');
                        s.push_str(
                            format!("------ Number of items :  {}   -------\n", self.total)
                                .as_str(),
                        );
                        s.push_str(
                            format!("------ Result limit    :  {}   -------\n", self.limit)
                                .as_str(),
                        );
                        s.push_str(
                            format!("------ Result offset   :  {}   -------\n", self.offset)
                                .as_str(),
                        );
                        s.push('\n');
                        s.push('\n');
                        s.push('\n');

                        section_track = true;
                        section_artist = false;
                        section_album = true;
                        items_count = self.offset
                    }
                }
                &SpotifySearchResultItem::Album { .. } => {
                    if section_album {
                        s.push_str("----------------------------------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push_str("----------------- Albums ---------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push_str("----------------------------------------\n");
                        s.push('\n');
                        s.push('\n');
                        s.push_str(
                            format!("------ Number of items :  {}   -------\n", self.total)
                                .as_str(),
                        );
                        s.push_str(
                            format!("------ Result limit    :  {}   -------\n", self.limit)
                                .as_str(),
                        );
                        s.push_str(
                            format!("------ Result offset   :  {}   -------\n", self.offset)
                                .as_str(),
                        );
                        s.push('\n');

                        section_track = true;
                        section_artist = true;
                        section_album = false;
                        items_count = self.offset
                    }
                }
            }
            items_count += 1;
            s.push_str(format!("****   {}\n", items_count).as_str());
            s.push_str(i.default_format().as_str());
            s.push('\n');
            s.push('\n');
        });
        s.push('\n');
        s.push('\n');
        s.push('\n');
        s
    }

    pub(crate) async fn show_spotify_search_result(&self, graphic: bool) {
        let mut section_track = true;
        let mut section_artist = true;
        let mut section_album = true;
        let mut items_count = self.offset;
        for item in self.items.iter() {
            match item {
                crate::libs::spotify::SpotifySearchResultItem::Track { .. } => {
                    if section_track {
                        print!("----------------------------------------\n");
                        print!("----------------------------------------\n");
                        print!("---------------  Tracks  ---------------\n");
                        print!("----------------------------------------\n");
                        print!("----------------------------------------\n");
                        print!("\n\n");
                        print!("------ Number of items :  {}   -------\n", self.total);
                        print!("------ result limit    :  {}   -------\n", self.limit);
                        print!("------ result offset   :  {}   -------\n", self.offset);
                        print!("\n");
                        section_track = false;
                        section_artist = true;
                        section_album = true;
                        items_count = self.offset
                    }
                }
                crate::libs::spotify::SpotifySearchResultItem::Artist { .. } => {
                    if section_artist {
                        print!("----------------------------------------\n");
                        print!("----------------------------------------\n");
                        print!("---------------- Artists ---------------\n");
                        print!("----------------------------------------\n");
                        print!("----------------------------------------\n");
                        print!("\n");
                        print!("\n");
                        print!("------ Number of items :  {}   -------\n", self.total);
                        print!("------ result limit    :  {}   -------\n", self.limit);
                        print!("------ result offset   :  {}   -------\n", self.offset);
                        print!("\n");
                        section_track = true;
                        section_artist = false;
                        section_album = true;
                        items_count = self.offset
                    }
                }
                crate::libs::spotify::SpotifySearchResultItem::Album { .. } => {
                    if section_album {
                        print!("----------------------------------------\n");
                        print!("----------------------------------------\n");
                        print!("----------------- Albums ---------------\n");
                        print!("----------------------------------------\n");
                        print!("----------------------------------------\n");
                        print!("\n");
                        print!("\n");
                        print!("------ Number of items :  {}   -------\n", self.total);
                        print!("------ result limit    :  {}   -------\n", self.limit);
                        print!("------ result offset   :  {}   -------\n", self.offset);
                        print!("\n");
                        section_track = true;
                        section_artist = true;
                        section_album = false;
                        items_count = self.offset
                    }
                }
            }
            items_count += 1;
            print!("****   {}\n", items_count);
            item.show_result(graphic).await;
            print!("\n\n");
        }
        print!("\n\n\n");
    }
}
