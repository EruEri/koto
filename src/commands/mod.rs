// /////////////////////////////////////////////////////////////////////////////////////////////
//                                                                                            //
// This file is part of Koto: A holdall music program                                         //
// Copyright (C) 2023 Yves Ndiaye                                                             //
//                                                                                            //
// Koto is free software: you can redistribute it and/or modify it under the terms            //
// of the GNU General Public License as published by the Free Software Foundation,            //
// either version 3 of the License, or (at your option) any later version.                    //
//                                                                                            //
// Koto is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;          //
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR           //
// PURPOSE.  See the GNU General Public License for more details.                             //
// You should have received a copy of the GNU General Public License along with Koto.         //
// If not, see <http://www.gnu.org/licenses/>.                                                //
//                                                                                            //
// /////////////////////////////////////////////////////////////////////////////////////////////

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
            KotoSubcommands::List(list) => list.run().await,
        }
    }
}

impl Koto {
    pub async fn run(self) {
        self.subcommand.run().await
    }
}
