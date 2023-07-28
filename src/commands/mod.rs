use clap::{Parser, Subcommand};

use crate::config::KOTO_VERSION;

pub mod create_m3u;
pub mod cuesheet;
pub mod edit;
pub mod init;
pub mod list;
pub mod search;

#[derive(Parser)]
#[clap(author, version = KOTO_VERSION, about, long_about = None)]
pub struct Koto {
    #[clap(subcommand)]
    pub subcommand: KotoSubcommands,
}

#[derive(Subcommand)]
pub enum KotoSubcommands {
    CreateM3U(create_m3u::CreateM3U),
    #[clap(subcommand)]
    CueSheet(cuesheet::CueSheetSubcommand),
    Edit(edit::Edit),
    Init(init::Init),
    Search(search::Search),
    List(list::List),
}

impl KotoSubcommands {
    pub async fn run(self) {
        match self {
            KotoSubcommands::CreateM3U(m3u) => m3u.run(),
            KotoSubcommands::CueSheet(cue) => cue.run().await,
            KotoSubcommands::Edit(edit) => edit.run(),
            KotoSubcommands::Init(init) => init.run(),
            KotoSubcommands::Search(search) => search.run().await,
            KotoSubcommands::List(list) => list.run(),
        }
    }
}

impl Koto {
    pub async fn run(self) {
        self.subcommand.run().await
    }
}
