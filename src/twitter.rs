use twitter_v2::TwitterApi;
use twitter_v2::authorization::{BearerToken};
use twitter_v2::query::{UserField};

pub async fn get_my_followers(twitter_bearer_token: &str) {
	let auth = BearerToken::new(twitter_bearer_token);
	let my_followers = TwitterApi::new(auth)
			.with_user_ctx()
			.get_my_followers()
			.user_fields([UserField::Username])
			.max_results(20)
			.send()
			.into_data();
}