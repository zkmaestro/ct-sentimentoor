use serde_derive::Deserialize;
use reqwest::Error;

#[derive(Debug, Deserialize)]
pub struct TwitterUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct APIData<T> {
    pub data: Vec<T>,
}

/**
 * Get a list of twitter users from a twitter list
 * 
 * # Arguments
 * 
 * * `bearer_token` - A twitter bearer token
 * * `list_id` - The id of the twitter list
 */
pub async fn get_list_members(bearer_token: String, list_id: String) -> Result<Vec<TwitterUser>, Error> {
    let client = reqwest::Client::new();
    let url = format!("https://api.twitter.com/2/lists/{}/members", list_id);
    println!("Request URl = {}", url);
    let response = client
            .get(&url)
            .bearer_auth(bearer_token)
            .send()
            .await?;
    let twitter_users: APIData<TwitterUser> = response.json().await?;
    Ok(twitter_users.data)
}

