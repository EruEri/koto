use clap::{ArgEnum, ArgGroup};
use rusqlite::params;
use std::{
    collections::HashSet,
    fmt::Display,
    fs::{read_dir, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
    time::Duration,
};
use tag_edit::{FlacTag, PictureFormat, ID3TAG};

use clap::{Parser, Subcommand};

use crate::{
    app_dir_pathbuf,
    bindings::libcuesheetmaker::cue_file_format,
    config::KOTO_VERSION,
    spotify::{Spotify, SpotifyIncludeGroupe, SpotifySearchType},
    sql::ArtistDB,
    util,
};

#[derive(Parser)]
#[clap(author, version = KOTO_VERSION, about, long_about = None)]
pub struct Main {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommands>,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// List the followed artists
    #[clap(group(
        ArgGroup::new("opt")
        .required(false)
        .args(&["delete", "add"])
        .conflicts_with("update")
        .conflicts_with("full")
    )) ]
    List {
        /// delete an artist to the list
        #[clap(short, long)]
        delete: Option<String>,
        /// add an artist to the list
        #[clap(short, long)]
        add: Option<String>,
        /// update the artist lastest album
        #[clap(short, long)]
        update: Option<Option<String>>,

        /// Display all the artist information
        #[clap(short, long)]
        full: bool,
        /// Filter with selected spotify id
        #[clap(short, long)]
        id: bool,
    },
    /// Search for an item

    // #[clap(subcommand_precedence_over_arg = true)]
    #[clap(subcommand_negates_reqs = true, args_conflicts_with_subcommands = true)]
    Search {
        #[clap(subcommand)]
        search_subcommand: Option<SearchSubCommand>,
        /// search for an artist
        #[clap(short, long)]
        artist: bool,
        /// search for an album
        #[clap(short = 'l', long)]
        album: bool,
        /// search for an track
        #[clap(short, long)]
        track: bool,

        /// market to look for
        #[clap(long)]
        market: Option<String>,

        /// limit the result
        /// MAX Value : 50
        #[clap(long)]
        limit: Option<u8>,

        /// Display graohic result (cover, picture, etc ...)
        #[clap(short, long)]
        graphic: bool,

        /// offset the result
        #[clap(long)]
        offset: Option<u32>,
        /// search item
        #[clap(required = true)]
        item: Option<String>,
    },
    /// Init koto with the spotify client credentials
    Init {
        /// Set the client id
        #[clap(long)]
        client_id: String,
        /// Set the client secret
        #[clap(long)]
        client_secret: String,
        /// Force the overwrite of credentials
        #[clap(short, long)]
        force: bool,
    },

    // #[clap(group(
    // ArgGroup::new("type")
    // .required(true)
    // .args(&["mp3", "flac"])
    // ))
    // ]
    /// Edit mp3 and flac file
    Edit {
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
    },

    /// Create Cue Sheet File
    CueSheet {
        #[clap(subcommand)]
        cs_subcommand: CueSheetSubcommand,
    },

    /// Create M3U playlist
    CreateM3U {
        /// Include files
        /// By default, matched files are [mp3, aiff, flac, wav, alac, ogg]
        #[clap(short, long)]
        include_extension: Vec<String>,

        /// Exclude files from being matched
        #[clap(short, long)]
        exclude_extention: Vec<String>,

        /// By default: Print to the standard output
        #[clap(short, long)]
        output: Option<String>,
        // #[clap(long)]
        // file_info: bool,

        // #[clap(long)]
        // stop_on_error: bool,
        directories: Vec<String>,
    },
}
#[derive(Subcommand)]
pub enum CueSheetSubcommand {
    /// Create the cue sheet by fechting the requiered information on the spotify api
    #[clap(group(
        ArgGroup::new("names")
            .required(false)
            .multiple(true)
            .args(&["artist", "album"])
            .conflicts_with("album-id")
            .requires_all(&["artist", "album"])
            
    )) ]
    Fetch {
        /// Artist name
        #[clap(short, long)]
        artist: Option<String>,
        /// Album name
        #[clap(long, alias = "al")]
        album: Option<String>,
        /// Album spotify Id
        #[clap(long, alias = "id")]
        album_id: Option<String>,
        /// Output file
        #[clap(short, long, help = "Output file [stdout if not present]")]
        output: Option<String>,

        #[clap(long, alias = "cfn", default_value = "")]
        cue_file_name: String,

        #[clap(short, long, arg_enum)]
        format: cue_file_format,
        /// Display album total duration
        #[clap(long)]
        total_duration: bool,
        /// output path where fetched album illustration will be created
        #[clap(short, long)]
        image: Option<String>,
    },

    /// Create the cue sheet by giving the requiered information throught the command line
    Make {},
}

#[derive(Subcommand)]
pub enum SearchSubCommand {
    // #[clap(groups(
    //     vec![
    //         ArgGroup::new("search_type")
    //         .required(false)
    //         .args(&["albums", "related_artists"])
    //     ]
    // )) ]
    /// Search content related to an artist
    #[clap(group(
            ArgGroup::new("search_type")
            .required(false)
            .args(&["albums"])
            .conflicts_with("related_artists")
    )) ]
    Artist {
        #[clap(short, long)]
        /// Search artist's albums
        albums: bool,
        #[clap(long)]
        /// Search related artists
        related_artists: bool,
        #[clap(long)]
        /// Search by artist id
        id: bool,
        #[clap(short, long)]
        /// Display graohic result (cover, picture, etc ...)
        graphic: bool,
        #[clap(short, long, default_value_t = 3)]
        /// Result limit
        limit: u32,
        /// Output column
        #[clap(short, long, default_value_t = 1)]
        column: usize,
        /// Search item
        artist: String,
    },
    /// Search content related to an album
    Album {},
    /// Search content related to a track
    Track {},
}
#[derive(Clone, Copy, Debug, ArgEnum)]
pub enum FileType {
    Mp3,
    Flac,
}
pub async fn run_list(
    delete: Option<String>,
    add: Option<String>,
    update: Option<Option<String>>,
    full: bool,
    id: bool,
) -> Option<()> {
    match (delete, add) {
        (Some(_), Some(_)) => unreachable!("delete and add cant be together in the same time"),
        (None, Some(add)) => run_list_modify(id, false, add).await,
        (Some(delete), None) => run_list_modify(id, true, delete).await,
        (None, None) => match update {
            None => run_list_show(full, None, false),
            Some(artist_opt) => run_list_update(artist_opt.map(|art| vec![art]), id).await,
        },
    }
}

pub async fn run_list_modify(id: bool, delete: bool, name: String) -> Option<()> {
    if !delete {
        let spotify = Spotify::init().await;
        let artist = ArtistDB::from_name(&name, id, &spotify).await?;
        match ArtistDB::insert_db(&artist) {
            Ok(u) => {
                if u == 1 {
                    println!(
                        "Operation Succesed\n{} added to the database",
                        artist.artist_name
                    );
                }
            }
            Err(e) => {
                println!("An error occured \n{}", e);
                return None;
            }
        }
    } else {
        let connection = ArtistDB::open().unwrap_or_else(|| panic!("Unable to open the database"));
        let field = if id {
            "artist_spotify_id"
        } else {
            "artist_name"
        };
        let sql_string = format!("DELETE FROM artist_table WHERE {} = ?1", field);
        match connection.execute(sql_string.as_str(), params![name]) {
            Ok(size) => {
                if size == 0 {
                    println!("No artist deleted")
                } else {
                    println!("{} artist deleted", size)
                }
            }
            Err(_) => {
                println!("An error Occurred");
                exit(1)
            }
        }
        let _ = connection.close();
    }

    println!("Done");
    Some(())
}

pub async fn run_list_update(names: Option<Vec<String>>, id: bool) -> Option<()> {
    let (mut artists, connection) =
        ArtistDB::fetch_all().unwrap_or_else(|| panic!("Unable to fetch into the database"));
    if let Some(name) = names {
        artists.retain(|artist| {
            name.contains(if id {
                &artist.artist_spotify_id
            } else {
                &artist.artist_name
            })
        });
    }
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    let token = crate::spotify::Token::new(client_id.as_str(), client_secret.as_str())
        .await
        .unwrap();
    let spotify = Spotify::new(&token);
    println!();
    println!("------------------------------");
    println!("-------- New realease --------");
    println!("------------------------------");
    println!("");
    for artist in artists.iter_mut() {
        tokio::time::sleep(Duration::from_millis(300)).await;
        let updated = artist.update(&spotify, &connection).await;
        if let Some(updated) = updated {
            if updated {
                println!("\n");
                println!("{}", artist.default_format());
                (0..3)
                    .into_iter()
                    .for_each(|_| println!("------------------"));
            }
        } else {
            println!("An error occured for : {}", artist.artist_name)
        }
    }
    println!("Done");
    Some(())
}

pub fn run_list_show(all_info: bool, names: Option<Vec<String>>, id: bool) -> Option<()> {
    let mut artists = ArtistDB::fetch_all()?.0;
    if let Some(name) = names {
        artists.retain(|artist| {
            name.contains(if id {
                &artist.artist_spotify_id
            } else {
                &artist.artist_name
            })
        });
    }

    for artist in artists {
        println!("\n");
        let artist_str = if all_info {
            artist.full_format()
        } else {
            artist.default_format()
        };
        println!("{}", artist_str);
        (0..3)
            .into_iter()
            .for_each(|_| println!("------------------"));
    }

    Some(())
}
// ------------------------- Init ------------------------- \\
pub fn run_init(client_id: String, client_secret: String, force: bool) -> Option<()> {
    if let (Some(_), Some(_)) = (
        std::env::var("CLIENT_ID").ok(),
        std::env::var("CLIENT_SECRET").ok(),
    ) {
        if !force {
            println!("Credentials already set");
            exit(1)
        }
    }
    let mut app_dir = app_dir_pathbuf();
    app_dir.push(".env");
    let mut env = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(app_dir)
        .ok()?;
    env.write(format!("CLIENT_ID={}\n", client_id).as_bytes())
        .ok()?;
    env.write(format!("CLIENT_SECRET={}\n", client_secret).as_bytes())
        .ok()?;
    Some(())
}
// ------------------------ Search ------------------------ \\

pub async fn run_search(
    search_subcommand: Option<SearchSubCommand>,
    artist: bool,
    album: bool,
    track: bool,
    market: Option<String>,
    limit: Option<u8>,
    graphic: bool,
    offset: Option<u32>,
    item: String,
) -> Option<()> {
    if let Some(command) = search_subcommand {
        return run_search_subcommand(command).await;
    }
    let mut ressource_types = vec![];
    if artist {
        ressource_types.push(SpotifySearchType::Artist)
    }
    if track {
        ressource_types.push(SpotifySearchType::Track)
    }
    if album {
        ressource_types.push(SpotifySearchType::Album)
    }
    let spotify = Spotify::init().await;
    let result = spotify
        .search(
            item.as_str(),
            ressource_types,
            market,
            limit.map(|l| if l > 50 { 50 } else { l }),
            offset,
            None,
        )
        .await?;
    for (_, ssr) in result.iter() {
        if ssr.items.is_empty() {
            println!("\n****   No Result   ****\n")
        } else {
            ssr.show_spotify_search_result(graphic).await;
        }
    }
    //println!("{:?}", result);

    Some(())
}

pub(crate) async fn run_search_subcommand(search: SearchSubCommand) -> Option<()> {
    match search {
        SearchSubCommand::Artist {
            albums,
            related_artists,
            id,
            graphic,
            limit,
            column,
            artist,
        } => run_artist_search(albums, related_artists, id, graphic, limit, column, artist).await,
        SearchSubCommand::Album {} => todo!(),
        SearchSubCommand::Track {} => todo!(),
    }
}

pub async fn run_artist_search(
    albums: bool,
    related_artists: bool,
    id: bool,
    graphic: bool,
    limit: u32,
    column: usize,
    query: String,
) -> Option<()> {
    let spotify = Spotify::init().await;
    let artist_id = if id {
        query
    } else {
        let result = spotify
            .search(
                query.as_str(),
                vec![SpotifySearchType::Artist],
                None,
                Some(1),
                Some(0),
                None,
            )
            .await
            .unwrap_or_else(|| {
                println!("Wrong Api Response");
                exit(1)
            });
        let items = result
            .get(&crate::spotify::SpotifySearchKey::Artists)
            .unwrap_or_else(|| {
                println!("Unable to get the artist");
                exit(1)
            });
        let mut vec_artist_id = items
            .items
            .iter()
            .filter_map(|ssri| match ssri {
                crate::spotify::SpotifySearchResultItem::Artist {
                    external_urls: _,
                    followers: _,
                    genres: _,
                    href: _,
                    id,
                    images: _,
                    name: _,
                    popularity: _,
                    artist_type: _,
                    uri: _,
                } => Some(id.clone()),
                _ => None,
            })
            .collect::<Vec<String>>();
        if !vec_artist_id.is_empty() {
            vec_artist_id.remove(0)
        } else {
            println!("No artist returned");
            exit(1)
        }
    };
    match (albums, related_artists) {
        (true, true) => unreachable!("Albums and Related are mutualy exclued"),
        (true, false) => {
            let albums = spotify
                .artist_album(
                    artist_id,
                    vec![SpotifyIncludeGroupe::Album],
                    Some(limit),
                    None,
                    None,
                )
                .await
                .unwrap_or_else(|| {
                    println!("Unable to fetch the related artist");
                    exit(1)
                });
        }
        (false, true) => {
            let related_artists = spotify
                .related_artists(&artist_id)
                .await
                .unwrap_or_else(|| {
                    println!("Unable to fetch the related artist");
                    exit(1)
                });
            return util::display_related_artist(&related_artists, column, limit as usize, graphic)
                .await;
        }
        (false, false) => {
            let artist = spotify.artist(artist_id.as_str()).await.unwrap_or_else(|| {
                println!("Unable to retrieve the artist");
                exit(1)
            });
            println!("Name  : {}", artist.name);
            println!("Genre : {}", artist.genres.join("\n        "));
            if let Some(map) = artist.images.get(0) {
                if let Some(url) = map.get("url") {
                    let url = url.as_str().unwrap();
                    let dyn_image = util::donwload_image(url).await;
                    if let Some(image) = dyn_image {
                        util::show_image(&image);
                    }
                }
            }
        }
    }
    Some(())
}

// ------------------------  Edit  ------------------------ \\
pub async fn run_edit(
    file_type: FileType,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    artist_album: Option<String>,
    year: Option<i16>,
    bpm: Option<u16>,
    track_position: Option<u16>,
    images: Option<Vec<String>>,
    output: Option<String>,
    file: String,
) -> Option<()> {
    match file_type {
        FileType::Mp3 => {
            let mut id3tag = ID3TAG::from_path(file.as_str())?;
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
                    let pathbuf = PathBuf::from_str(&image.as_str()).ok()?;
                    let extension = pathbuf
                        .extension()
                        .map(|os| os.to_str())
                        .unwrap_or_else(|| Some(""))?;
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
            Some(())
        }
        FileType::Flac => {
            let mut flac = FlacTag::from_path(file.as_str())?;
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
                    let pathbuf = PathBuf::from_str(&image.as_str()).ok()?;
                    let extension = pathbuf
                        .extension()
                        .map(|os| os.to_str().unwrap().to_string())
                        .unwrap_or_else(|| String::new());

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
            Some(())
        }
    }
}

// ------------------------  CueSheet  ------------------------ \\

pub async fn run_cuesheet(cs_subcommand: CueSheetSubcommand) {
    match cs_subcommand {
        CueSheetSubcommand::Fetch {
            artist,
            album,
            album_id,
            output,
            cue_file_name,
            format,
            total_duration,
            image,
        } => {
            run_cuesheet_fetch(
                artist,
                album,
                album_id,
                output,
                cue_file_name,
                format,
                total_duration,
                image,
            )
            .await
        }
        CueSheetSubcommand::Make {} => todo!(),
    }
}

pub async fn run_cuesheet_fetch(
    artist: Option<String>,
    album: Option<String>,
    album_id: Option<String>,
    output: Option<String>,
    cue_file_name: String,
    format: cue_file_format,
    total_duration: bool,
    image: Option<String>,
) {
    let spotify = Spotify::init().await;

    let album_id = if let Some(id) = album_id {
        id
    } else {
        let artist = artist.unwrap();
        let album = album.unwrap();
        match spotify
            .search(
                format!("{} {}", album, artist).as_str(),
                vec![SpotifySearchType::Album],
                None,
                Some(1),
                None,
                None,
            )
            .await
        {
            None => {
                println!("No Resultat");
                exit(1)
            }
            Some(result) => {
                let data = result
                    .get(&crate::spotify::SpotifySearchKey::Albums)
                    .unwrap_or_else(|| {
                        println!("Unable to fetch the artist");
                        exit(1)
                    });
                data.items.iter().filter_map(|ssri| match ssri {
                    crate::spotify::SpotifySearchResultItem::Album {
                        album_type: _,
                        artists: _,
                        external_urls: _,
                        available_markets: _,
                        href: _,
                        id,
                        images: _,
                        name: _,
                        release_date: _,
                        release_date_precision: _,
                        total_tracks: _,
                        a_type: _,
                        uri: _,
                    } => Some(id.clone()),
                    _ => None,
                })
            }
            .collect::<Vec<String>>()
            .first()
            .unwrap_or_else(|| {
                println!("Unable to get the artist Id");
                exit(1)
            })
            .clone(),
        }
    };
    if let Err(e) = util::cuesheet_from_album_id(
        cue_file_name,
        format,
        output,
        album_id.as_str(),
        total_duration,
        image,
    )
    .await
    {
        println!("{}", e);
        exit(1)
    }
}

// ------------------------  CreateM3U  ------------------------ \\

const DEFAULT_AUDIO_FILE_EXTENSION: &[&'static str] =
    &["mp3", "aiff", "flac", "wav", "alac", "ogg"];

struct M3UPlaylist {
    items: Vec<String>,
}

impl Display for M3UPlaylist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "#EXTM3U\n\n");
        let _ = self.items.iter().for_each(|path| {
            let _ = writeln!(f, "{}\n", path);
        });
        write!(f, "")
    }
}

impl M3UPlaylist {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn _append_path<T: AsRef<Path>>(&mut self, path: T) {
        self.items.push(
            path.as_ref()
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        )
    }

    fn extract_audio_files(&mut self, extensions: &HashSet<String>, path: &PathBuf) {
        let files_in_dir = read_dir(path).unwrap();
        files_in_dir.for_each(|entry| {
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();
            if metadata.is_file()
                && entry
                    .path()
                    .extension()
                    .map(|osstr| extensions.contains(&osstr.to_str().unwrap().to_lowercase()))
                    .unwrap_or(false)
            {
                self.items.push(
                    entry
                        .path()
                        .canonicalize()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
            } else if metadata.is_dir() {
                self.extract_audio_files(extensions, &entry.path())
            } else {
            }
        })
    }

    fn append_sub_dir(&mut self, extensions: &HashSet<String>, path: &PathBuf) {
        self.extract_audio_files(extensions, path);
        self.items.sort()
    }
}

pub fn run_create_m3u(
    include_extension: Vec<String>,
    exclude_extension: Vec<String>,
    output: Option<String>,
    directories: Vec<String>,
) {
    let mut extensions = HashSet::new();
    let _ = DEFAULT_AUDIO_FILE_EXTENSION.iter().for_each(|extension| {
        let _ = extensions.insert(extension.to_string().to_lowercase());
    });
    let _ = include_extension.iter().for_each(|extension| {
        let _ = extensions.insert(extension.to_owned().to_lowercase());
    });

    let _ = exclude_extension.iter().for_each(|extension| {
        let _ = extensions.remove(&extension.to_lowercase());
    });

    let mut m3u_playlist = M3UPlaylist::new();
    for raw_path in directories.iter() {
        let path = PathBuf::from(raw_path);
        m3u_playlist.append_sub_dir(&extensions, &path)
    }

    if m3u_playlist.items.is_empty() {
        println!("No matched files");
        return;
    }

    match output {
        None => println!("{}", m3u_playlist),
        Some(output) => {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output)
                .unwrap_or_else(|error| panic!("{}", error));
            let _ = file.write_all(m3u_playlist.to_string().as_bytes());
            ()
        }
    }
}
