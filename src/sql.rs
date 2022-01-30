use std::fmt::Display;

use rusqlite::{Connection, params, ToSql, types::{ToSqlOutput, Value}};

use crate::spotify::Spotify;
const DB_PATH : &'static str = "new_song_db.sqlite";
const FETCH_ALL_QUERY : &'static str = "Select * from artist_table";


#[derive(Debug)]
pub struct ArtistDB {
    pub (crate) id : u32,
    pub (crate) artist_name : String,
    pub (crate) artist_spotify_id : String,
    pub (crate) last_album : String,
    pub (crate) last_album_release_date : Date,
    pub (crate) last_album_spotify_id : String,
    pub (crate) last_album_url : String,
}

impl ArtistDB {
    pub async fn from_name(name : &String, id : bool, spotify : &Spotify) -> Option<Self> {
        if id {
            let artist = spotify.artist(name).await?;
            let lastest_album = spotify.artist_lastest_album(artist.id.as_str()).await?;
            Some(Self {
                            id : 0,
                            artist_name : artist.name,
                            artist_spotify_id : artist.id,
                            last_album : lastest_album.name,
                            last_album_release_date : Date::from_str(&lastest_album.release_date)?,
                            last_album_spotify_id : lastest_album.id,
                            last_album_url : lastest_album.external_urls.get("spotify")?.clone()
                        })
        }else {
            unimplemented!()
        }
    }
}

impl ArtistDB {
    pub fn fetch_all() -> Option<(Vec<ArtistDB>, Connection)>{
        let connection = Connection::open(DB_PATH).ok()?;
        let mut stmt = connection.prepare(FETCH_ALL_QUERY).ok()?;
        let artists = stmt.query_map([], |r| {
            
            Ok(ArtistDB {
                            id : r.get(0)?,
                            artist_name : r.get(1)?,
                            artist_spotify_id : r.get(2)?,
                            last_album : r.get(3)?,
                            last_album_release_date : Date::from_str(&r.get(4)?).unwrap(),
                            last_album_spotify_id : r.get(5)?,
                            last_album_url : r.get(6)?,
                        })
        }).ok()?
        .filter_map(|res| res.ok())
        .collect::<Vec<ArtistDB>>();
        drop(stmt);
        Some((artists, connection))
    }

    pub fn open() -> Option<Connection> {
        Connection::open("new_song_db_copie.sqlite").ok()
    }

    pub async fn update(&mut self, spotify : &Spotify, connection : &Connection) -> Option<bool> {
        let lastest = spotify.artist_lastest_album(self.artist_spotify_id.as_str()).await?;
        let latest_realease_date = Date::from_str(&lastest.release_date)?;
        if  latest_realease_date > self.last_album_release_date {
            self.last_album = lastest.name;
            self.last_album_spotify_id = lastest.id;
            self.last_album_release_date = latest_realease_date;
            self.last_album_url = lastest.external_urls.get("spotify")?.clone();
            let _ = Self::update_db(self, connection)?;
            Some(true)
        }else {
            Some(false)
        }
        
    }

    pub fn insert_db(artist : &Self, database : &Connection) -> Option<()> {
        database.execute( "INSERT INTO artist_table 
        (artist_name, artist_spotify_id, last_album, last_album_release_date, last_album_spotify_id, last_album_url)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
         params![artist.artist_name, artist.artist_spotify_id, artist.last_album, 
         artist.last_album_release_date, artist.last_album_spotify_id, artist.last_album_url
         ]).ok()?;

         Some(())
    }
    pub fn update_db(artist : &Self, database : &Connection) -> Option<()> {
        database.execute("UPDATE artist_table SET last_album = ?1, last_album_release_date = ?2, last_album_spotify_id = ?3, last_album_url = ?4 WHERE id = ?5;"
        , params![artist.last_album, artist.last_album_release_date, artist.last_album_spotify_id, artist.last_album_url, artist.id]).ok()?;

        Some(())
    }

    pub fn default_format(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("***   Artist Name   : {}   \n", self.artist_name).as_str());
        s.push_str(format!("***   Last Album    : {}   \n", self.last_album).as_str());
        s.push_str(format!("***   Realease Date : {}   \n", self.last_album_release_date).as_str());
        s
    }

    pub fn full_format(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("***   ID            : {}   \n", self.id).as_str());
        s.push_str(format!("***   Artist Name   : {}   \n", self.artist_name).as_str());
        s.push_str(format!("***   Artist ID     : {}   \n", self.artist_spotify_id).as_str());
        s.push_str(format!("***   Last Album    : {}   \n", self.last_album).as_str());
        s.push_str(format!("***   Last album ID : {}   \n", self.artist_name).as_str());
        s.push_str(format!("***   Realease Date : {}   \n", self.last_album_release_date).as_str());
        s.push_str(format!("***   Album Url     : {}   \n", self.last_album_url).as_str());
        s
    }
}


#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Date {
    day : u8,
    month : u8,
    year : i16,
}


extern "C" {
    fn date_now() -> Date;
}

impl Date {
    pub fn date_now() -> Self {
        unsafe {
            date_now()
        }
    }

    pub fn unix_epoch() -> Self {
        Self {
            day : 1,
            month : 1,
            year : 1970
        }
    }

    

    pub fn from_str<'a>(date : &String) -> Option<Self> {
        let date_tuple = date.split("-").collect::<Vec<&str>>();
        let year = date_tuple.get(0)?.parse::<i16>().ok()?;
        let month = date_tuple.get(1).map(|f|  {
            if f.starts_with("0") { f.strip_prefix("0").unwrap()} else { f }
        })?.parse::<u8>().ok()?;
        let day = date_tuple.get(2).map(|f|  {
            if f.starts_with("0") { f.strip_prefix("0").unwrap()} else { f }
        })?.parse::<u8>().ok()?;
        Some(Self {
                    year,
                    month,
                    day
                })
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.year == other.year {
            if self.month == other.month {
                Some(self.day.cmp(&other.day))
            }else {
                Some(self.month.cmp(&other.month))
            }
        }else {
            Some(self.year.cmp(&other.year))
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fill_month = if self.month < 10 { "0" } else { "" };
        let fill_day = if self.day < 10 { "0" } else { "" };
        write!(f, "{}-{}{}-{}{}",self.year, fill_month,self.month, fill_day,self.day)
    }
}

impl ToSql for Date {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}