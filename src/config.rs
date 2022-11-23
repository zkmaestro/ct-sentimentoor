use std::fs::{File};
use std::error::Error;
use std::io::BufReader;
use serde_derive::{Deserialize, Serialize};

const CONFIG_FILE: &str = "./sentimentoor.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub twitter_bearer_token : String,
    pub twitter_list_id : String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let file = File::open(CONFIG_FILE)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}