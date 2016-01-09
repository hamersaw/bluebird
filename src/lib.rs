extern crate crypto;
extern crate hyper;
extern crate rand;
extern crate rustc_serialize;
extern crate time;

pub mod stream;

use std::collections::{BTreeMap,HashMap};
use rand::Rng;

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use rustc_serialize::base64::{self,ToBase64};

pub struct RequestConfig {
    parameters: HashMap<String,String>,
    oauth_config: OAuthConfig,
}

impl RequestConfig {
    pub fn new(parameters: HashMap<String,String>, oauth_config: OAuthConfig) -> RequestConfig {
        RequestConfig {
            parameters: parameters,
            oauth_config: oauth_config,
        }
    }

    pub fn get_parameter_count(&self) -> usize {
        self.parameters.len()
    }

    pub fn get_data_body(&self) -> String {
        let mut data_string = String::new();
        for (i, (key, value)) in self.parameters.iter().enumerate() {
            if i != 0 {
                data_string.push_str("&");
            }

            data_string.push_str(&format!("{}={}", key, value)[..]);
        }

        data_string
    }

    pub fn get_oauth_signature(&self) -> String {
        let mut map = BTreeMap::new();
        for (key, value) in self.parameters.iter() {
            map.insert(key.clone(), value.clone());
        }

        map.insert("oauth_consumer_key".to_string(), self.oauth_config.consumer_key.clone());
        map.insert("oauth_nonce".to_string(), self.oauth_config.nonce.clone());
        map.insert("oauth_signature_method".to_string(), "HMAC-SHA1".to_string());
        map.insert("oauth_timestamp".to_string(), self.oauth_config.timestamp.to_string());
        map.insert("oauth_token".to_string(), self.oauth_config.access_token.clone());
        map.insert("oauth_version".to_string(), "1.0".to_string());

        let mut parameter_string = String::new();
        for (key, value) in map.iter() {
            parameter_string.push_str(&format!("&{}={}", key, value)[..]);
        }

        let signature_base_string = format!("POST&{}&{}", percent_encode("https://stream.twitter.com/1.1/statuses/filter.json".to_string()), percent_encode(parameter_string[1..].to_string()));
        let signing_key = format!("{}&{}", percent_encode(self.oauth_config.consumer_secret.clone()), percent_encode(self.oauth_config.access_token_secret.clone()));

        let mut hmac = Hmac::new(Sha1::new(), &signing_key.into_bytes());
        hmac.input(&signature_base_string.into_bytes());
        hmac.result().code().to_base64(base64::STANDARD)
    }

    pub fn get_authorization_header(&self) -> String {
        format!("OAuth \
            oauth_consumer_key=\"{}\", \
            oauth_nonce=\"{}\", \
            oauth_signature=\"{}\", \
            oauth_signature_method=\"HMAC-SHA1\", \
            oauth_timestamp=\"{}\", \
            oauth_token=\"{}\", \
            oauth_version=\"1.0\"", 
            percent_encode(self.oauth_config.consumer_key.clone()),
            self.oauth_config.nonce,
            percent_encode(self.get_oauth_signature()),
            self.oauth_config.timestamp,
            percent_encode(self.oauth_config.access_token.clone()),
        )
    }
}

pub struct OAuthConfig {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    nonce: String,
    timestamp: i64,
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

pub fn percent_encode(string: String) -> String {
    string.chars().map(|x| {
        match x {
            '0'...'9' | 'A'...'Z' | 'a'...'z' | '-' | '.' | '_' | '~' => format!("{}", x),
            _ => format!("%{:X}", x as u8),
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("Ladies + Gentleman".to_string()), "Ladies%20%2B%20Gentleman".to_string());
        assert_eq!(percent_encode("Dogs, Cats, & Mice!".to_string()), "Dogs%2C%20Cats%2C%20%26%20Mice%21".to_string());
    }
}
