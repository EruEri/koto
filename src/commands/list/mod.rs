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

pub mod db;

use clap::{ArgGroup, Parser};

use crate::{
    config::{check_credential_exist, extend_env},
    libs::spotify::Spotify,
};

use self::db::{Artist, Artists};

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
    pub async fn run(self) {
        let () = extend_env();
        let () = match check_credential_exist() {
            true => (),
            false => return,
        };
        match (&self.add, &self.delete) {
            (Some(_), Some(_)) => unreachable!("Are mutual excluded"),
            (None, Some(name)) => self.run_delete(name).await,
            (Some(name), None) => self.run_add(name).await,
            (None, None) => match &self.update {
                None => self.run_show(),
                Some(artist_opt) => self.run_update(artist_opt).await,
            },
        }
    }

    async fn run_update(&self, artist_opt: &Option<String>) {
        let mut db = match Artists::deserialize() {
            Some(db) => db,
            None => {
                println!("Cannot deserialize");
                return;
            }
        };
        db.update(self.id, artist_opt).await;
        db.save();
    }

    async fn run_add(&self, name: &String) {
        let mut db = match Artists::deserialize() {
            Some(db) => db,
            None => {
                println!("Cannot deserialize");
                return;
            }
        };
        let spotify = Spotify::init().await;
        let artist = match Artist::from_name(&spotify, name, self.id).await {
            Some(a) => a,
            None => {
                println!("Artist not found");
                return;
            }
        };
        db.add(artist);
        db.save();
    }

    async fn run_delete(&self, name: &String) {
        let mut db = match Artists::deserialize() {
            Some(db) => db,
            None => {
                println!("Cannot deserialize");
                return;
            }
        };
        db.delete(self.id, name);
        db.save();
    }

    fn run_show(&self) {
        let db = match Artists::deserialize() {
            Some(db) => db,
            None => {
                println!("Cannot deserialize");
                return;
            }
        };
        db.show(self.full)
    }
}
