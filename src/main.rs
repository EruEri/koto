use clap::StructOpt;
use commands::Koto;


pub mod commands;
pub mod config;
mod bindings;
mod spotify;
mod util;

#[tokio::main]
async fn main() {
    let koto = Koto::parse();
    koto.run().await
}
