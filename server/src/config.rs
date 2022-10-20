use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct StartupConfig {
    pub address: String,
    pub max_players: i32,

    pub ext_dir: String,
    pub data_dir: String,

    pub online_mode: bool
}

impl StartupConfig {
    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path.clone()).unwrap();
        let mut str = String::new();
        file.read_to_string(&mut str).unwrap();
        let config: StartupConfig = toml::from_str(&str).unwrap();
        config
    }

    pub fn generate() -> String {
        let c = Self {
            address: String::from("0.0.0.0:25565"),
            max_players: 20,
            ext_dir: String::from("./exts"),
            data_dir: String::from("./data"),

            online_mode: false
        };

        toml::to_string(&c).unwrap()
    }
}