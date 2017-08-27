#![feature(proc_macro)]

extern crate url;
extern crate hyper;
extern crate rustc_serialize;
extern crate oauthcli;

extern crate serde;
extern crate serde_json;

use super::{Tweet};
use hyper::Client;
use hyper::header::Headers;
use std::io::Read;
use std::iter::Iterator;

header! { (Authorization, "Authorization") => [String] }
header! { (Accept, "Accept") => [String] }
header! { (ContentType, "Content-Type") => [String] }

fn get_own_feed_as_json_string(since_id: Option<u64>) -> String {

    //Change these values to your real Twitter API credentials
	let consumer_key = "bMKb6A4X8fWYVEeAvQ0U82Je7";
	let consumer_secret = "srvdUQEPw9q2HK6qcX8b9h479KsDKzbuwmJrObZyuN3TXTTuKu";
	let token = "313560220-ZBwe0sO5Df41ysbCB16LlJdOFA2jRTC7HHIat3g6";
	let token_secret = "ZgSPU46z9WU8HoPG3g6AhI5W17Ohs6cehjosgbeE9ZbLO";

    //Track words
    //let params: Vec<(String, String)> = vec![("track".to_string(), "london".to_string())];
    let mut params: Vec<(String, String)> = vec![];

    if let Some(id) = since_id {
    	params.push(("since_id".to_owned(), id.to_string()))
    }

    //https://api.twitter.com/1.1/search/tweets.json?q=#ceta&count=4
	let url = "https://api.twitter.com/1.1/statuses/home_timeline.json";

	let header = oauthcli::authorization_header(
	    "GET",
	    url::Url::parse(url).unwrap(),
	    None, // Realm
	    consumer_key,
	    consumer_secret,
	    Some(token),
	    Some(token_secret),
	    oauthcli::SignatureMethod::HmacSha1,
	    &oauthcli::timestamp(),
	    &oauthcli::nonce(),
	    None, // oauth_callback
	    None, // oauth_verifier
	    params.clone().into_iter()
	);
	println!("auth header {:?}", header);

    let client = Client::new();

	let mut headers = Headers::new();
	headers.set(Authorization(header.to_owned()));
	//headers.set(Accept("*/*".to_owned()));
	//headers.set(ContentType("application/x-www-form-urlencoded".to_owned()));

	let param_string: String = params.iter().map(|p| p.0.clone() + &"=".to_string() + &p.1).collect::<Vec<String>>().join("&");

	let mut res: hyper::client::response::Response = client.get(url).headers(headers).body(&param_string).send().unwrap();

	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();

	body
}

#[derive(Deserialize, Debug)]
struct DeserializedTweet {
	text: String,
	user: DeserializedUser,
	id: u64
}

#[derive(Deserialize, Debug)]
struct DeserializedUser {
	name: String
}

#[repr(C)]
pub struct TwitterAPIClient {

}

impl super::TwitterClient for TwitterAPIClient {
	fn get(&mut self, since_id: Option<u64>) -> Vec<Tweet> {
        let json = get_own_feed_as_json_string(since_id);
        let dtweets: Vec<DeserializedTweet> = serde_json::from_str(&json).unwrap();
        let tweets = dtweets.into_iter()
        	.map(|dt| Tweet::new(dt.user.name, dt.text, dt.id))
        	.collect();

        tweets
    }
}