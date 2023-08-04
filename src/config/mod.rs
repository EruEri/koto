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

use xdg::BaseDirectories;

pub const KOTO_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const KOTO_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const KOTO_DB_NAME: &'static str = "db.json";
pub const KOTO_ENV: &'static str = ".env";

pub fn koto_base_dir() -> BaseDirectories {
    xdg::BaseDirectories::with_prefix(KOTO_NAME)
        .unwrap_or_else(|e| panic!("Cannot create xdg dirs: {}", e))
}

pub fn extend_env() {
    let koto_dir = koto_base_dir();
    let path = match koto_dir.find_config_file(KOTO_ENV) {
        Some(path) => path,
        None => {
            println!("No env file, You should maybe run {} init", KOTO_NAME);
            return;
        }
    };
    let _ = dotenv::from_path(path);
    let _ = dotenv::dotenv();
}

pub fn check_credential_exist() -> bool {
    if let None = std::env::var("CLIENT_ID").ok() {
        println!("CLIENT_ID key not found\nYou should maybe run koto init");
        return false;
    }
    if let None = std::env::var("CLIENT_SECRET").ok() {
        println!("CLIENT_SECRET key not found\nYou should maybe run koto init");
        return false;
    }

    return true;
}
