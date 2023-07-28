use clap::{ArgGroup, Parser};

/// List the followed artists
#[derive(Debug, Parser)]
#[clap(group(
    ArgGroup::new("opt")
    .required(false)
    .args(&["delete", "add"])
    .conflicts_with("update")
    .conflicts_with("full")
)) ]
pub struct List {
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
}

impl List {
    pub fn run(self) {}
}
