use std::path::Path;
use std::fs::{File};
use serde_json;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub twitter_api_key : String, 
    pub twitter_api_secret : String,
}

impl Config {
    pub fn read_config(config_file_path: &Path) -> Option<Config> {
        let mut file = match File::open(config_file_path) {
            Ok(f) => f,
            Err(_) => return None,
        };
        serde_json::from_reader(&mut file).ok()
    }
}