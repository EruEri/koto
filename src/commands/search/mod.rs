use clap::{Parser, Subcommand};

use crate::spotify::{Spotify, SpotifySearchType};

pub mod album;
pub mod artist;
pub mod track;

/// Search for an item
// #[clap(subcommand_precedence_over_arg = true)]
#[derive(Parser)]
#[clap(subcommand_negates_reqs = true, args_conflicts_with_subcommands = true)]
pub struct Search {
    #[clap(subcommand)]
    search_subcommand: Option<SearchSubCommand>,
    /// search for an artist
    #[clap(short, long)]
    artist: bool,
    /// search for an album
    #[clap(long)]
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
}

#[derive(Subcommand)]
pub enum SearchSubCommand {
    Artist(artist::Artist),
    Album(album::Album),
    Track(track::Track),
}

impl SearchSubCommand {
    pub async fn run(self) {
        match self {
            SearchSubCommand::Artist(artist) => artist.run().await,
            SearchSubCommand::Album(album) => album.run().await,
            SearchSubCommand::Track(track) => track.run().await,
        }
    }
}

impl Search {
    pub async fn run(self) {
        match self.search_subcommand {
            Some(sub) => sub.run().await,
            None => self.run_search().await,
        }
    }

    pub async fn run_search(self) {
        let Search {
            search_subcommand: _,
            artist,
            album,
            track,
            market,
            limit,
            graphic,
            offset,
            item,
        } = self;
        let item = item.unwrap_or("".into());
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
            .await;
        let result = match result {
            Some(res) => res,
            None => {
                println!("No result");
                return;
            }
        };
        for (_, ssr) in result.iter() {
            if ssr.items.is_empty() {
                println!("\n****   No Result   ****\n")
            } else {
                ssr.show_spotify_search_result(graphic).await;
            }
        }
    }
}
