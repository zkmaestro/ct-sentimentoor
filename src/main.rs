use reqwest::Error;

mod config;
use crate::config::Config;

mod twitter;
use crate::twitter::TwitterUser;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new().unwrap();
    let bearer_token = config.twitter_bearer_token;
    let list_id = config.twitter_list_id;
    let list_members: Vec<TwitterUser> = twitter::get_list_members(bearer_token, list_id).await?;
    println!("There are {} twitter members in the list", list_members.len());
    for twitter_user in list_members {
        println!("Twitter User = {:?}", twitter_user);
    }
    Ok(())
}
