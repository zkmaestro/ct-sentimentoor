use std::collections::HashMap;
use log::{info};
use reqwest::Error;
use chrono::{DateTime, Utc};

mod config;
use crate::config::Config;

mod twitter;
use crate::twitter::{Tweet, TwitterUser};

mod sentiment;
use crate::sentiment::get_sentiment_for_tweets;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new().unwrap();
    let bearer_token = config.twitter_bearer_token;
    let list_id = config.twitter_list_id;

    info!("Calling get_list_members");
    let list_members: Vec<TwitterUser> = twitter::get_list_members(&bearer_token, &list_id).await?;
    let list_members_count = list_members.len();
    info!("There are {} twitter members in the list", list_members_count);

    // iterate each member and get the last 24 hours of tweets, then determine the sentiment of each user
    // based on the content of their tweets
    let now: DateTime<Utc> = Utc::now();
    let day = "2022-11-29";//now.format("%Y-%m-%d").to_string();
    let mut total_tweets = 0;
    let mut user_sentiment_map = HashMap::new();
    for member in list_members {
        info!("Getting tweets for {}", member.name);
        let tweets:Vec<Tweet> = twitter::get_user_tweets_for_day(&bearer_token, &member.id, &day).await?;
        info!("{} has {} tweets", member.name, tweets.len());
        total_tweets += tweets.len();
        // now determine the sentiment of this users tweets
        let sentiment = get_sentiment_for_tweets(tweets);
        info!("{} has a sentiment of {}", member.name, sentiment);
        user_sentiment_map.insert(member.name, sentiment.to_string());
    }
    info!("On {}, there were {} tweets in total. Thats and average of {} per user.", day, total_tweets, total_tweets/list_members_count);
    for (name, sentiment) in &user_sentiment_map {
        info!("{} has a sentiment of {}", name, sentiment);
    }
    // get the average sentiment of the list
    let mut total_sentiment = 0.0;
    for sentiment in user_sentiment_map.values() {
        total_sentiment += sentiment.parse::<f64>().unwrap();
    }
    let average_sentiment = total_sentiment / list_members_count as f64;
    info!("The total sentiment of the list across {} users is {}. The average sentiment of the list is {}.", list_members_count, average_sentiment, total_sentiment); 
    // get the median sentiment of the list
    let mut sentiments:Vec<f64> = user_sentiment_map.values().map(|s| s.parse::<f64>().unwrap()).collect();
    sentiments.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_sentiment = sentiments[sentiments.len() / 2];
    info!("The median sentiment of the list is {}", median_sentiment);

    /*
    let user_id = config.twitter_user_id;
    info!("Calling get_user_follows");
    let follow_members: Vec<TwitterUser> = twitter::get_user_follows(&bearer_token, &user_id).await?;
    info!("There are {} twitter members in the list", follow_members.len());
    */
    Ok(())
}
