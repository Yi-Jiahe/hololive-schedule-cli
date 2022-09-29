use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub holodex_api_token: String
}

impl Config {
    pub fn new() -> Config {
        Config{
            holodex_api_token: "".to_string()
        }
    }

    pub fn from_file(path: &str) -> Result<Config, std::io::Error> {
        let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

        let mut contents = String::new();
        config_file.read_to_string(&mut contents)?;
        let config = toml::from_str(&contents).unwrap();
        Ok(config)
    }

    pub fn write_to_file(&self, path: &str) {
        let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path).unwrap();
        file.write_all(toml::to_string(&self).unwrap().as_bytes());
    }
}