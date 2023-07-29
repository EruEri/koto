use std::{env::current_dir, fs::OpenOptions, io::Write};

use clap::Parser;

use crate::{config::{koto_base_dir, KOTO_NAME}, commands::list::db::Artists};

#[derive(Parser)]
/// Init koto with the spotify client credentials
pub struct Init {
    /// Set the client id
    #[clap(long)]
    client_id: String,
    /// Set the client secret
    #[clap(long)]
    client_secret: String,
    /// Force the overwrite of credentials
    #[clap(short, long)]
    force: bool,
}

impl Init {
    pub fn run(self) {
        let Init {
            client_id,
            client_secret,
            force,
        } = self;
        if let (Some(_), Some(_)) = (
            std::env::var("CLIENT_ID").ok(),
            std::env::var("CLIENT_SECRET").ok(),
        ) {
            if !force {
                println!("Credentials already set");
                return;
            }
        }
        let pwd = match current_dir() {
            Ok(path) => path,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        let koto_dir = koto_base_dir();
        // let _config_path = match koto_dir.create_config_directory(&pwd) {
        //     Ok(path) => path,
        //     Err(e) => {
        //         println!("Error {}", e);
        //         return;
        //     }
        // };

        // let _data_path = match koto_dir.create_data_directory(&pwd) {
        //     Ok(path) => path,
        //     Err(e) => {
        //         println!("Error {}", e);
        //         return;
        //     }
        // };

        let db_path = match koto_dir.place_data_file("db.json") {
            Ok(db_path) => db_path,
            Err(e) => {
                println!("Error {}", e);
                return;
            },
        };
        let db = Artists::default();
        let mut file = match OpenOptions::new().create(true).truncate(true).write(true).open(db_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error {}", e);
                return;
            },
        };
        let () = match serde_json::to_writer_pretty(file, &db) {
            Ok(()) => (),
            Err(e) => {
                println!("Error {}", e);
                return;
            },
        };
        let env = match koto_dir.place_config_file(".env") {
            Ok(path) => path,
            Err(e) => {
                println!("Error {}", e);
                return;
            }
        };
        let file_opt = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(env);
        let mut env_file = match file_opt {
            Ok(file) => file,
            Err(e) => {
                println!("Error {}", e);
                return;
            }
        };
        let e = env_file
            .write(format!("CLIENT_ID={}\n", client_id).as_bytes())
            .and_then(|_| env_file.write(format!("CLIENT_SECRET={}\n", client_secret).as_bytes()))
            .err();
        let () = match e {
            Some(error) => {
                println!("Error {}", error);
                return;
            }
            None => (),
        };
        println!("{} Initialized", KOTO_NAME)
    }
}
