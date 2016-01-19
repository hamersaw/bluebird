extern crate bluebird;
use bluebird::Client;

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

    let client = Client::new(consumer_key.trim(), consumer_secret.trim(), access_token.trim(), access_token_secret.trim());

    match client.home_timeline(Some("5"), None, None) {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    /*match client.lookup_users(Some("twitterapi,twitter"), None) {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.open_filter_stream(None, Some("twitter"), None) {
        Ok(rx) => {
            while let Ok(tweet) = rx.recv() {
                println!("{}", tweet);
            }
        },
        Err(e) => panic!("{}", e),
    }

    match client.search_users("twitterapi", None, None) {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.show_user(Some("twitterapi"), None) {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.update_status("Testing the rust twitter api \"bluebird\" (https://github.com/hamersaw/bluebird)") {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }

    match client.user_timeline(Some("twitterapi"), None, Some("5"), None, None) {
        Ok(json) => println!("{}", json),
        Err(e) => panic!("{}", e),
    }*/
}
