use clap::StructOpt;
use commands::Koto;

mod bindings;
mod command;
pub mod commands;
pub mod config;
mod spotify;
mod sql;
mod util;

#[tokio::main]
async fn main() {
    let koto = Koto::parse();
    koto.run().await
}
