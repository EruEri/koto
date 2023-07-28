use std::{
    collections::HashSet,
    fmt::Display,
    fs::{read_dir, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;

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

/// Create M3U playlist
#[derive(Parser)]
pub struct CreateM3U {
    /// Include files
    /// By default, matched files are [mp3, aiff, flac, wav, alac, ogg]
    #[clap(short, long)]
    include_extension: Vec<String>,

    /// Exclude files from being matched
    #[clap(short, long)]
    exclude_extension: Vec<String>,

    /// By default: Print to the standard output
    #[clap(short, long)]
    output: Option<String>,
    // #[clap(long)]
    // file_info: bool,

    // #[clap(long)]
    // stop_on_error: bool,
    directories: Vec<String>,
}

impl CreateM3U {
    pub fn run(self) -> () {
        let CreateM3U {
            include_extension,
            exclude_extension,
            output,
            directories,
        } = self;
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
}
