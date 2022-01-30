use std::{fmt::{format, Display}, collections::HashMap};
use serde::{Deserialize, Serialize};

use base64::encode;
use reqwest::{StatusCode, RequestBuilder};
use serde_json::Value;

use crate::sql::Date;

pub(crate) async fn get_access_token(client_id: &str, client_secret: &str) -> Option<Token> {
    let creditential = format!("{}:{}", client_id, client_secret);
    let encoded = format!("Basic {}", encode(creditential));
    let request = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", encoded)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials")
        .send()
        .await;
    match request {
        Err(_) => None,
        Ok(r) => {
            if r.status() != StatusCode::OK {
                None
            } else {
                let value = r.json::<Value>().await.ok()?;
                let access_token = value.as_object()?["access_token"].as_str()?.to_string();
                let token_type = value.as_object()?["token_type"].as_str()?.to_string();
                let expire_in: u32 = value.as_object()?["expires_in"].as_u64()? as u32;
                Some(Token {
                    access_token,
                    token_type,
                    expire_in,
                })
            }
        }
    }
}

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
    pub fn new(token : &Token) -> Self {
        Self {
            token: token.clone()
        }
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

    fn create_url(end_point : &String, r_type : &SpotifyRessourceType, spotify_ids : Vec<String>, market : Option<String>, limit : Option<u32>, offset : Option<u32>, included_genre : Vec<SpotifyIncludeGroupe>) -> RequestBuilder{
        let ids = if spotify_ids.len() > 1 { let s = String::from("?ids=") + &spotify_ids.join("%2C").to_string(); s } else { spotify_ids.first().unwrap().clone() };
        let base_url = format!("{}{}",end_point, ids);
        let base_url = match r_type {
            SpotifyRessourceType::ArtistAlbum => format!("{}{}", base_url, "/albums"),
            SpotifyRessourceType::ArtistTopTrack => format!("{}{}", base_url, "/top-tracks"),
            SpotifyRessourceType::AlbumTrack => format!("{}{}", base_url, "/tracks"),
            SpotifyRessourceType::RelatedArtist => format!("{}{}", base_url, "/related-artists"),
            _ => base_url
        };
        let req = reqwest::Client::new();
        let mut builder = req.get(base_url);
        if let  Some(market) = market {
            builder = builder.query(&[("market", market)]);
        }
        if let Some(offset) = offset {
            builder = builder.query(&[("offset", offset)]);
        }
        if let Some(limit) = limit {
            builder = builder.query(&[("limit", limit)])
        }

        if !included_genre.is_empty() {
            let genre = included_genre.iter().map( |i| i.to_string()).collect::<Vec<String>>().join(",");
            builder = builder.query(&[("include_groups", genre)]);
        }
        builder
    }

    fn setup_url_request(&self, r_type : &SpotifyRessourceType, spotify_ids : Vec<String>, market : Option<String>, limit : Option<u32> ,offset : Option<u32>, included_genre : Vec<SpotifyIncludeGroupe>) -> RequestBuilder {
        let end_point = &match r_type {
            &SpotifyRessourceType::Album | &SpotifyRessourceType::AlbumTrack => Self::album_end_point(),
            &SpotifyRessourceType::Artist | &SpotifyRessourceType::ArtistAlbum | &SpotifyRessourceType::ArtistTopTrack | &SpotifyRessourceType::RelatedArtist => Self::artist_end_point(),
            &SpotifyRessourceType::Track => Self::track_end_point(),
            &SpotifyRessourceType::AudioFeature => Self::audio_feature_end_point(),
            &SpotifyRessourceType::AudioAnalysis => Self::audio_analysis_end_point(),
        };
        let rb = Self::create_url(end_point, r_type, spotify_ids, market, limit, offset, included_genre);
        let rb = rb.header("Authorization", format!("Bearer {}", self.token.access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");
        rb
    }
    fn setup_search_request(&self, query : &str, item_type : Vec<SpotifySearchType>, market : Option<&str>, limit : Option<u32>, offset : Option<u32>, include_external : Option<bool>) -> RequestBuilder {
        let req = reqwest::Client::new();
        let mut builder = req.get(Self::search_end_point());
        let item_type = item_type.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(",");
        builder = builder.query(&[("q", query)]).query(&[("type", item_type)]);
        if let  Some(market) = market {
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
        builder = builder.header("Authorization", format!("Bearer {}", self.token.access_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json");  
        builder
    }
    pub async fn search(&self, query : &str, item_type : Vec<SpotifySearchType>, market : Option<&str>, limit : Option<u32>, offset : Option<u32>, include_external : Option<bool>) -> Option<Value> {
        self.setup_search_request(query, item_type, market, limit, offset, include_external).send().await.ok()?.json::<Value>().await.ok()
    }
    pub async fn artist(&self, artist_id : &str) -> Option<Artist> {
        let rb = self.setup_url_request(&SpotifyRessourceType::Artist, vec![artist_id.into()], None, None, None, vec![]);
        if let Ok(response ) = rb.send().await {
            response.json::<Artist>().await.ok()
        }else { None } 
    }

    pub async fn artists(&self, artist_ids : Vec<String>) -> Option<Vec<Value>> {
        let rb = self.setup_url_request(&SpotifyRessourceType::Artist, artist_ids, None, None, None, vec![]);
        if let Ok(response ) = rb.send().await {
            response.json::<Vec<Value>>().await.ok()
        }else { None } 
    }
    pub async fn artist_album(&self, artist_id : String, included_genre : Vec<SpotifyIncludeGroupe>, limit : Option<u32>, market : Option<String>, offset : Option<u32>) -> Option<Album> {
        let rb = self.setup_url_request(&SpotifyRessourceType::ArtistAlbum, vec![artist_id], market, limit, offset, included_genre);
        if let Ok(response) = rb.send().await {
            response.json::<Album>().await.ok()
        }else { None }
    }
    pub async fn related_artists(&self, artist_ids : Vec<String>) -> Option<Value> {
        let rb = self.setup_url_request(&SpotifyRessourceType::RelatedArtist, artist_ids, None, None,None, vec![]);
        if let Ok(response ) = rb.send().await {
            Some(response.json::<Value>().await.ok()?)
        }else { None } 
    }
}

impl Spotify {
    pub async fn artist_lastest_album(&self, artist_id : &str) -> Option<AlbumItems> {
        let mut album = self.artist_album(artist_id.into(), vec![
            SpotifyIncludeGroupe::Album, SpotifyIncludeGroupe::Single
        ], Some(40), None, Some(0)).await?;
        album.items.sort_by(|a,b| {
            let date1 = Date::from_str(&a.release_date).unwrap_or(Date::unix_epoch());
            let date2 = Date::from_str(&b.release_date).unwrap_or(Date::unix_epoch());
            date2.partial_cmp(&date1).unwrap()
        });
        let items = album.items.remove(0);
        Some(items)
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
    AudioAnalysis
}


#[derive(Debug, PartialEq, Eq)]
pub enum SpotifyIncludeGroupe {
    Single,
    Album,
    AppearsOn,
    Compilation
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
pub enum  SpotifySearchType {
    Album,
    Artist,
    Track
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


#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub (crate) external_urls : HashMap<String, String>,
    pub (crate) followers : HashMap<String, Value>,
    pub (crate) genres : Vec<String>,
    pub (crate) href : String,
    pub (crate) id : String,
    pub (crate) images : Vec<HashMap<String, Value>>,
    pub (crate) name : String,
    pub (crate) popularity: i32,
    #[serde(rename = "type")]
    pub (crate) artist_type : String,
    pub (crate) uri : String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    pub ( crate) href : String,
    pub ( crate) items : Vec<AlbumItems>,
    pub ( crate) limit: u8,
    pub ( crate) next : Option<String>,
    pub ( crate) offset : u8,
    pub ( crate) previous : Option<String>,
    pub ( crate) total : u8
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumItems {
    pub (crate) album_group : String,
    pub (crate) album_type : String,
    pub (crate) artists : Vec<Value>,
    pub (crate) available_markets : Vec<String>,
    pub (crate) external_urls : HashMap<String, String>,
    pub (crate) href : String,
    pub (crate) id : String,
    pub (crate) images : Vec<HashMap<String, Value>>,
    pub (crate) name : String,
    pub (crate) release_date : String,
    pub (crate) release_date_precision : String,
    pub (crate) total_tracks : u8,
    #[serde(rename = "type")]
    pub (crate) r#type : String,
    pub (crate) uri : String
}



// pub struct AlbumItems {
//     album_group : String,
//     album_type : String,
//     artists : Vec<Value>,
//     available_markets : String,
//     external_urls : HashMap<String, String>,
//     href : String,
//     id : String,
//     images : Vec<HashMap<String, Value>>,
//     name : String,
//     release_date : String,
//     release_date_precision : String,
//     total_tracks : u8,
//     #[serde(rename = "type")]
//     r_type : String,
//     uri : String

// }