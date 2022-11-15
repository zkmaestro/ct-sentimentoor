extern crate serde_json;

use std::path::Path;

mod config;

const CONFIG_FILE: &str = "./sentimentoor.json";

fn main() {
    let conf_path = Path::new(CONFIG_FILE);

    let config = match config::Config::read_config(&conf_path) {
        Some(v) => v,
        None => panic!("Cannot find config file"),
    };

    let twitter_api_key = config.twitter_api_key;
    let twitter_api_secret = config.twitter_api_secret;
}
