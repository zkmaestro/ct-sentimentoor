use vader_sentiment as vader;
use crate::twitter::{Tweet};
use log::{info};

/**
 * Get the sentiment of a specific tweets
 * 
 * # Arguments
 * 
 * * `test` - Text of a tweet
 */
pub fn get_sentiment(text: &str) -> f64 {
		let analyzer = vader::SentimentIntensityAnalyzer::new();
		let sentiment = analyzer.polarity_scores(text);
		let mut compound: f64 = sentiment.get("compound").unwrap().to_owned();
		if compound.is_nan() {
				compound = 0.0;
		}
		info!("Sentiment for {} is {}", text, compound);
		compound
}

/**
 * Get the sentiment of a list of tweets
 * 
 * # Arguments
 * 
 * * `tweets` - A list of tweets
 */
pub fn get_sentiment_for_tweets(tweets: Vec<Tweet>) -> f64 {
		let mut total_sentiment = 0.0;
		let total_tweets = tweets.len();
		for tweet in tweets {
				total_sentiment += get_sentiment(&tweet.text);
		}
		total_sentiment / total_tweets as f64
}