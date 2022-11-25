use serde_derive::Deserialize;
use reqwest::Error;
use log::{info, error};

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
 * Get a list of twitter users from a twitter list
 * 
 * # Arguments
 * 
 * * `bearer_token` - A twitter bearer token
 * * `list_id` - The id of the twitter list
 */
pub async fn get_list_members(bearer_token: &str, list_id: &str) -> Result<Vec<TwitterUser>, Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.twitter.com/2/lists/{}/members", list_id);
    let response = client
            .get(&url)
            .bearer_auth(bearer_token)
            .send()
            .await?;
		let list_members_response_text = response.text().await.unwrap();
		info!("[{}] Response: {}", url, list_members_response_text);
		let mut twitter_users: APIResponse<TwitterUser> = match serde_json::from_str::<APIResponse<TwitterUser>>(&list_members_response_text) {
			Ok(twitter_users) => twitter_users,
			Err(e) => {
				error!("[{}] Error: {}", url, e);
				panic!("Error: {}", e);
			}
		};
		while twitter_users.meta.next_token != "" {
			let url = format!("https://api.twitter.com/2/lists/{}/members?pagination_token={}", list_id, twitter_users.meta.next_token);
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
		}
    Ok(twitter_users.data)
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
		let client = reqwest::Client::new();
		let url = format!("https://api.twitter.com/2/users/{}/following", user_id);
		info!("Calling {}", url);
		let response = client
						.get(&url)
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
		while twitter_users.meta.next_token != "" {
			let url = format!("https://api.twitter.com/2/users/{}/following?pagination_token={}", user_id, twitter_users.meta.next_token);
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
		}
		Ok(twitter_users.data)
}

