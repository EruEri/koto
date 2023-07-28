use clap::Subcommand;

pub mod cuesheet_fetch;
pub mod cuesheet_make;

#[derive(Subcommand)]
/// Create the cue sheet by fechting the requiered information on the spotify api
pub enum CueSheetSubcommand {
    Fetch(cuesheet_fetch::CueSheetFetch),
    Make(cuesheet_make::CueSheetMake),
}

impl CueSheetSubcommand {
    pub async fn run(self) {
        match self {
            CueSheetSubcommand::Fetch(fetch) => fetch.run().await,
            CueSheetSubcommand::Make(make) => make.run(),
        }
    }
}
