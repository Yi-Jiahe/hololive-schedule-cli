use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ConfigWriteError {
    TomlSerializeError {
        cause: toml::ser::Error
    },
    FileWriteError {
        cause: std::io::Error
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub holodex_api_token: String,
    pub format: Format,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Format {
    pub channel_name_col_length: usize,
    pub stream_title_name_col_length: usize,
}

impl Config {
    pub fn new() -> Config {
        Config {
            holodex_api_token: "".to_string(),
            format: Format {
                channel_name_col_length: 25,
                stream_title_name_col_length: 70,
            },
        }
    }

    pub fn from_file(path: &str) -> Result<Config, std::io::Error> {
        let mut config_file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut contents = String::new();
        config_file.read_to_string(&mut contents)?;
        let config = toml::from_str(&contents).unwrap();
        Ok(config)
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), ConfigWriteError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();

        match file.write_all(match toml::to_string(&self) {
            Ok(s) => s,
            Err(e) => return Err(ConfigWriteError::TomlSerializeError{
                cause: e
            })
        }.as_bytes()) {
            Ok(()) => (),
            Err(e) => return Err(ConfigWriteError::FileWriteError{
                cause: e
            })
        };
        
        Ok(())
    }
}
