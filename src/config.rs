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
}