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

use clap::StructOpt;
use commands::Koto;

pub mod commands;
pub mod config;
mod libs;

#[tokio::main]
async fn main() {
    let koto = Koto::parse();
    koto.run().await
}
