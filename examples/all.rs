extern crate bluebird;
use bluebird::Client;
use bluebird::request::BluebirdRequest;

use std::io;

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

    let client = Client::new(
        consumer_key.trim().to_string(),
        consumer_secret.trim().to_string(),
        access_token.trim().to_string(),
        access_token_secret.trim().to_string()
    );

    //USER
    match client.lookup_users(Some("twitterapi,twitter".to_string()), None).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.search_users("twitterapi".to_string(), None, None).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.show_user(None, Some("twitterapi".to_string())).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    //STATUS
    match client.home_timeline(Some("10".to_string()), None, None).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.update_status("Testing the rust twitter api \"bluebird\" (https://github.com/hamersaw/bluebird)".to_string()).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.user_timeline(None, Some("twitterapi".to_string()), Some("10".to_string()), None, None).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    //STREAM
    match client.open_filter_stream(None, Some("twitter".to_string()), None).exec() {
        Ok(rx) => {
            while let Ok(tweet) = rx.recv() {
                println!("{}", tweet);
            }
        },
        Err(e) => panic!("{}", e),
    }
}
