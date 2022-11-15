extern crate serde_json;

use std::path::Path;

mod config;
mod twitter;

const CONFIG_FILE: &str = "./sentimentoor.json";

fn main() {
    let conf_path = Path::new(CONFIG_FILE);

    let config = match config::Config::read_config(&conf_path) {
        Some(v) => v,
        None => panic!("Cannot find config file"),
    };

    twitter::get_my_followers(config.twitter_bearer_token);
}
