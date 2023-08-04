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
