use rand::{self,Rng};
//use rustc_serialize::base64::{self,ToBase64};
use time;

pub struct OAuthConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
    pub nonce: String,
    pub timestamp: i64,
}

impl OAuthConfig {
    pub fn new(consumer_key: String, consumer_secret: String, access_token: String, access_token_secret: String) -> OAuthConfig {
        OAuthConfig {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            access_token: access_token,
            access_token_secret: access_token_secret,
            nonce: rand::thread_rng().gen_ascii_chars().take(32).collect::<String>(),
            timestamp: time::now_utc().to_timespec().sec,
        }
    }
}
