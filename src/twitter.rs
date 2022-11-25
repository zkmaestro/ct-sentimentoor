use serde_derive::Deserialize;
use reqwest::Error;
use log::{info, error};

#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TwitterUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct APIResponseMeta {
		pub result_count: u32,
		#[serde(default)]
		pub next_token: String,
		#[serde(default)]
		pub oldest_id: String,
		#[serde(default)]
		pub newest_id: String,
}

#[derive(Deserialize, Debug)]
pub struct APIResponse<T> {
    pub data: Vec<T>,
		pub meta: APIResponseMeta,
}

/**
 * Get a specific user's tweets for a specific day. If the day is now provided do today
 * 
 * # Arguments
 * 
 * * `bearer_token` - A twitter bearer token
 * * `user_id` - The id of the twitter list
 */
pub async fn get_user_tweets_for_day(bearer_token: &str, user_id: &str, date: &str) -> Result<Vec<Tweet>, Error> {
		let qs_date_filter = format!("?start_time={}T00:00:00.000Z&end_time={}T23:30:00.000Z", date, date);
		let url = format!("https://api.twitter.com/2/users/{}/tweets{}", user_id, qs_date_filter);
		get_tweet_based_get_request(bearer_token, &url).await
}

/**
 * Get a list of twitter users from a twitter list
 * 
 * # Arguments
 * 
 * * `bearer_token` - A twitter bearer token
 * * `list_id` - The id of the twitter list
 */
pub async fn get_list_members(bearer_token: &str, list_id: &str) -> Result<Vec<TwitterUser>, Error> {
    let url = format!("https://api.twitter.com/2/lists/{}/members", list_id);
		get_twitter_user_based_get_request(bearer_token, &url).await
}

/**
 * Get a list of twitter users from a twitter list
 * 
 * # Arguments
 * 
 * * `bearer_token` - A twitter bearer token
 * * `user_id` - The id of the user to get the follow list of
 */
pub async fn get_user_follows(bearer_token: &str, user_id: &str) -> Result<Vec<TwitterUser>, Error> {
		let url = format!("https://api.twitter.com/2/users/{}/following", user_id);
		get_twitter_user_based_get_request(bearer_token, &url).await
}

/**
 * Generic function to call the twitter api that returns a list of twitter users
 * 
 * # Arguments
 * 
 * * `bearer_token` - A twitter bearer token
 * * `url` - url of the api that returns a list of twitter users
 */
async fn get_twitter_user_based_get_request(bearer_token: &str, url: &str) -> Result<Vec<TwitterUser>, Error> {
    let client = reqwest::Client::new();
		info!("Calling {}", url);
		let response = client
						.get(url)
						.bearer_auth(bearer_token)
						.send()
						.await?;
		let twitter_users_response_text = response.text().await.unwrap();
		info!("[{}] Response: {}", url, twitter_users_response_text);
		let mut twitter_users: APIResponse<TwitterUser> = match serde_json::from_str::<APIResponse<TwitterUser>>(&twitter_users_response_text) {
			Ok(twitter_users) => twitter_users,
			Err(e) => {
				error!("[{}] Error: {}", url, e);
				panic!("Error: {}", e);
			}
		};
		let mut next_token = twitter_users.meta.next_token;
		while next_token != "" {
			let qs_operator: &str = if url.contains("?") { "&" } else { "?" };
			let url = format!("{}{}pagination_token={}", url, qs_operator, next_token);
			info!("[{}] Calling next page", url);
			let response = client
					.get(&url)
					.bearer_auth(bearer_token)
					.send()
					.await?;
			let twitter_users_next_text = response.text().await.unwrap();
			info!("[{}] Paginated Response: {}", url, twitter_users_next_text);
			let twitter_users_next: APIResponse<TwitterUser> = match serde_json::from_str::<APIResponse<TwitterUser>>(&twitter_users_next_text) {
				Ok(twitter_users_next) => twitter_users_next,
				Err(e) => {
					error!("[{}] Error: {}", url, e);
					panic!("Error: {}", e);
				}
			};
			twitter_users.data.extend(twitter_users_next.data);
			next_token = twitter_users_next.meta.next_token;
		}
		Ok(twitter_users.data)
}

/**
 * Generic function to call the twitter api that returns a list of tweets
 * 
 * # Arguments
 * 
 * * `bearer_token` - A twitter bearer token
 * * `url` - url of the api that returns a list of twitter users
 */
async fn get_tweet_based_get_request(bearer_token: &str, url: &str) -> Result<Vec<Tweet>, Error> {
    let client = reqwest::Client::new();
		info!("Calling {}", url);
		let response = client
						.get(url)
						.bearer_auth(bearer_token)
						.send()
						.await?;
		let tweet_response_text = response.text().await.unwrap();
		info!("[{}] Response: {}", url, tweet_response_text);
		let mut tweets: APIResponse<Tweet> = match serde_json::from_str::<APIResponse<Tweet>>(&tweet_response_text) {
			Ok(tweets) => tweets,
			Err(e) => {
				error!("[{}] Error: {}", url, e);
				panic!("Error: {}", e);
			}
		};
		let mut next_token = tweets.meta.next_token;
		while next_token != "" {
			let qs_operator: &str = if url.contains("?") { "&" } else { "?" };
			let url = format!("{}{}pagination_token={}", url, qs_operator, next_token);
			info!("[{}] Calling next page", url);
			let response = client
					.get(&url)
					.bearer_auth(bearer_token)
					.send()
					.await?;
			let tweets_next_text = response.text().await.unwrap();
			info!("[{}] Paginated Response: {}", url, tweets_next_text);
			let tweets_next: APIResponse<Tweet> = match serde_json::from_str::<APIResponse<Tweet>>(&tweets_next_text) {
				Ok(tweets_next) => tweets_next,
				Err(e) => {
					error!("[{}] Error: {}", url, e);
					panic!("Error: {}", e);
				}
			};
			tweets.data.extend(tweets_next.data);
			next_token = tweets_next.meta.next_token;
		}
		Ok(tweets.data)
}

