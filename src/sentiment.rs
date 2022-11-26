use vader_sentiment as vader;
use crate::twitter::{Tweet};
use log::{info};


pub fn get_sentiment(text: &str) -> f64 {
		let analyzer = vader::SentimentIntensityAnalyzer::new();
		let sentiment = analyzer.polarity_scores(text);
		let compound: f64 = sentiment.get("compound").unwrap().to_owned();
		info!("Sentiment for {} is {}", text, compound);
		compound
}

pub fn get_sentiment_for_tweets(tweets: Vec<Tweet>) -> f64 {
		let mut total_sentiment = 0.0;
		let total_tweets = tweets.len();
		for tweet in tweets {
				total_sentiment += get_sentiment(&tweet.text);
		}
		total_sentiment / total_tweets as f64
}