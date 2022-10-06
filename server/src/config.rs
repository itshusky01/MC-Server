use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct StartupConfig {
    pub name: String,
    pub address: String,
    pub max_players: i32,
    // fake_players: isize, //TODO

    pub ext_dir: String,
}

impl StartupConfig {
    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path.clone()).unwrap();
        let mut str = String::new();
        file.read_to_string(&mut str).unwrap();
        let config: StartupConfig = toml::from_str(&str).unwrap();
        config
    }
}