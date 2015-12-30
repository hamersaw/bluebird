use rand::{self,Rng};
use rustc_serialize::base64::{self,ToBase64};
use time;

pub struct OAuthConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
    pub nonce: String,
    pub timestamp: f64,
}

impl OAuthConfig {
    pub fn new(consumer_key: String, consumer_secret: String, access_token: String, access_token_secret: String) -> OAuthConfig {
        OAuthConfig {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            access_token: access_token,
            access_token_secret: access_token_secret,
            nonce: generate_nonce(),
            timestamp: time::precise_time_s(),
        }
    }
}

pub fn generate_nonce() -> String {
    let mut random = rand::thread_rng();    
    let mut nonce = [0u8; 32];
    random.fill_bytes(&mut nonce[..]);
    let nonce_string = nonce.to_base64(base64::STANDARD);

    nonce_string.chars().map(|x| {
        match x {
            '0'...'9' | 'A'...'Z' | 'a'...'z' | '-' | '.' | '_' | '~' => format!("{}", x),
            _ => "".to_string(),
        }
    }).collect()
}
