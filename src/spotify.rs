use std::{fmt::{format, Display}, collections::HashMap};
use serde::{Deserialize, Serialize};

use base64::encode;
use reqwest::{StatusCode, RequestBuilder};
use serde_json::Value;

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
pub(crate) struct Token {
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

pub(crate) struct Spotify {
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

    fn create_url(end_point : &String, r_type : &SpotifyRessourceType, spotify_ids : Vec<String>, market : Option<String>, offset : Option<u32>, included_genre : Vec<SpotifyIncludeGroupe>) -> RequestBuilder{
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

        if !included_genre.is_empty() {
            let genre = included_genre.iter().map( |i| i.to_string()).collect::<Vec<String>>().join(",");
            builder = builder.query(&[("include_groups", genre)]);
        }
        builder
    }

    fn setup_url_request(&self, r_type : &SpotifyRessourceType, spotify_ids : Vec<String>, market : Option<String>, offset : Option<u32>, included_genre : Vec<SpotifyIncludeGroupe>) -> RequestBuilder {
        let end_point = &match r_type {
            &SpotifyRessourceType::Album | &SpotifyRessourceType::AlbumTrack => Self::album_end_point(),
            &SpotifyRessourceType::Artist | &SpotifyRessourceType::ArtistAlbum | &SpotifyRessourceType::ArtistTopTrack | &SpotifyRessourceType::RelatedArtist => Self::artist_end_point(),
            &SpotifyRessourceType::Track => Self::track_end_point(),
            &SpotifyRessourceType::AudioFeature => Self::audio_feature_end_point(),
            &SpotifyRessourceType::AudioAnalysis => Self::audio_analysis_end_point(),
        };
        let rb = Self::create_url(end_point, r_type, spotify_ids, market, offset, included_genre);
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
        let rb = self.setup_url_request(&SpotifyRessourceType::Artist, vec![artist_id.into()], None, None, vec![]);
        if let Ok(response ) = rb.send().await {
            response.json::<Artist>().await.ok()
        }else { None } 
    }

    pub async fn artists(&self, artist_ids : Vec<String>) -> Option<Vec<Value>> {
        let rb = self.setup_url_request(&SpotifyRessourceType::Artist, artist_ids, None, None, vec![]);
        if let Ok(response ) = rb.send().await {
            response.json::<Vec<Value>>().await.ok()
        }else { None } 
    }
    pub async fn related_artists(&self, artist_ids : Vec<String>) -> Option<Value> {
        let rb = self.setup_url_request(&SpotifyRessourceType::RelatedArtist, artist_ids, None, None, vec![]);
        if let Ok(response ) = rb.send().await {
            Some(response.json::<Value>().await.ok()?)
        }else { None } 
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
            SpotifyIncludeGroupe::Album => "appears_on",
            SpotifyIncludeGroupe::AppearsOn => "album",
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