use clap::StructOpt;
use commands::Koto;

mod bindings;
pub mod commands;
pub mod config;
mod libs;

#[tokio::main]
async fn main() {
    let koto = Koto::parse();
    koto.run().await
}
