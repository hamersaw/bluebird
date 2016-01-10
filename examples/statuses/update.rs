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
    let update_config = bluebird::statuses::create_update_config(
        "Testing the rust twitter API \"bluebird\" (https://github.com/hamersaw/bluebird)".to_string() //status
    );

    match bluebird::statuses::post_status_update(&update_config, &oauth_config) {
        Ok(_) => println!("successfully update status"),
        Err(e) => panic!("{}", e),
    }
}
