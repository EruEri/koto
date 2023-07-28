use clap::{Parser, Subcommand};

pub mod create_m3u;
pub mod cuesheet;
pub mod edit;
pub mod init;
pub mod list;
pub mod search;


#[derive(Parser)]
#[clap(author, version = super::KOTO_VERSION, about, long_about = None)]
pub struct Koto {
    #[clap(subcommand)]
    pub subcommand: Option<KotoSubcommands>,
}


#[derive(Subcommand)]
pub enum KotoSubcommands {
    CreateM3U(create_m3u::CreateM3U)
}