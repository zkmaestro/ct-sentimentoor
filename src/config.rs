use env_logger::Builder;
use log::LevelFilter;
use std::fs::{File};
use std::error::Error;
use std::io::BufReader;
use serde_derive::{Deserialize, Serialize};

const CONFIG_FILE: &str = "./sentimentoor.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub twitter_bearer_token : String,
    pub twitter_list_id : String,
    pub twitter_user_id : String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let file = File::open(CONFIG_FILE)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        let log_level: LevelFilter = match config.log_level.as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        };
        Builder::new().filter_level(log_level).init();
        Ok(config)
    }
}