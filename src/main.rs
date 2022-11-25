use log::{info};
use reqwest::Error;

mod config;
use crate::config::Config;

mod twitter;
use crate::twitter::{Tweet, TwitterUser};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new().unwrap();
    let bearer_token = config.twitter_bearer_token;
    let list_id = config.twitter_list_id;

    info!("Calling get_list_members");
    let list_members: Vec<TwitterUser> = twitter::get_list_members(&bearer_token, &list_id).await?;
    info!("There are {} twitter members in the list", list_members.len());

    // iterate each member and get the last 24 hours of tweets, then determine the sentiment of each user
    // based on the content of their tweets
    for member in list_members {
        info!("Getting tweets for {}", member.name);
        let tweets:Vec<Tweet> = twitter::get_user_tweets_for_day(&bearer_token, &member.id, "2022-11-24").await?;
        info!("{} has {} tweets", member.name, tweets.len());
    }

    /*
    let user_id = config.twitter_user_id;
    info!("Calling get_user_follows");
    let follow_members: Vec<TwitterUser> = twitter::get_user_follows(&bearer_token, &user_id).await?;
    info!("There are {} twitter members in the list", follow_members.len());
    */
    Ok(())
}
