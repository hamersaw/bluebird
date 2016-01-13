use std::io;

extern crate bluebird;
use bluebird::OAuthConfig;

fn main() {
    let (mut consumer_key, mut consumer_secret, mut access_token, mut access_token_secret)  = (String::new(), String::new(), String::new(), String::new());
    println!("enter consumer key:");
    let _ = io::stdin().read_line(&mut consumer_key).unwrap();

    println!("enter consumer secret:");
    let _ = io::stdin().read_line(&mut consumer_secret).unwrap();

    println!("enter access token:");
    let _ = io::stdin().read_line(&mut access_token).unwrap();

    println!("enter access token secret:");
    let _ = io::stdin().read_line(&mut access_token_secret).unwrap();

    let oauth_config = OAuthConfig::new(consumer_key.trim().to_string(), consumer_secret.trim().to_string(), access_token.trim().to_string(), access_token_secret.trim().to_string());
    let filter_config = bluebird::filter_stream::create_config(
        None, //follow users ex. "12345,3425"
        Some("twitter".to_string()), //track keywords ex. "twitter,facebook,linkedin"
        None, //location bounding boxes ex. "-74,40,-73,41"
    );

    let rx = match bluebird::filter_stream::open_stream(&filter_config, &oauth_config) {
        Ok(rx) => rx,
        Err(e) => panic!("{}", e),
    };

    while let Ok(tweet) = rx.recv() {
        println!("{}", tweet);
    }
}
