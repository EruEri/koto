use rusqlite::Connection;
const DB_PATH : &'static str = "new_song_db.sqlite";
const FETCH_ALL_QUERY : &'static str = "Select * from artist_table";


#[derive(Debug)]
pub struct ArtistDB {
    id : u32,
    artist_name : String,
    artist_spotify_id : String,
    last_album : String,
    last_album_release_date : Date,
    last_album_spotify_id : String,
    last_album_url : String,
} 

impl ArtistDB {
    pub fn fetch_all() -> Option<Vec<ArtistDB>>{
        let connection = Connection::open(DB_PATH).ok()?;
        let mut stmt = connection.prepare(FETCH_ALL_QUERY).ok()?;
        println!("smrmr");
        let artists = stmt.query_map([], |r| {
            
            Ok(ArtistDB {
                            id : r.get(0)?,
                            artist_name : r.get(1)?,
                            artist_spotify_id : r.get(2)?,
                            last_album : r.get(3)?,
                            last_album_release_date : Date::from_str(r.get(4)?).unwrap(),
                            last_album_spotify_id : r.get(5)?,
                            last_album_url : r.get(6)?,
                        })
        }).ok()?
        .filter_map(|res| res.ok())
        .collect::<Vec<ArtistDB>>();
        Some(artists)
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

    pub fn from_str(date : String) -> Option<Self> {
        let date_tuple = date.split("-").collect::<Vec<&str>>();
        let year = date_tuple.get(0)?.parse::<i16>().ok()?;
        let month = date_tuple.get(1)?.parse::<u8>().ok()?;
        let day = date_tuple.get(2)?.parse::<u8>().ok()?;
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
