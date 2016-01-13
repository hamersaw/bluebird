extern crate bluebird;
use bluebird::client::Client;
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

    //SHOW USER
    match client.show_user(None, Some("hamersaw".to_string())).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    //UPDATE STATUS
    match client.update_status("Testing the rust twitter api \"bluebird\" (https://github.com/hamersaw/bluebird)".to_string()).exec() {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    //OPEN FILTER STREAM
    match client.open_filter_stream(None, Some("twitter".to_string()), None).exec() {
        Ok(rx) => {
            while let Ok(tweet) = rx.recv() {
                println!("{}", tweet);
            }
        },
        Err(e) => panic!("{}", e),
    }
}
