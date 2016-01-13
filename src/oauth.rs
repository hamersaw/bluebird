use percent_encode;

use rand::{self,Rng};
use std::collections::BTreeMap;
use time;

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use rustc_serialize::base64::{self,ToBase64};

#[derive(Clone)]
pub struct OAuthConfig {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
}

impl OAuthConfig {
    pub fn new(consumer_key: String, consumer_secret: String, access_token: String, access_token_secret: String) -> OAuthConfig {
        OAuthConfig {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            access_token: access_token,
            access_token_secret: access_token_secret,
        }
    }

    pub fn get_authorization_header(&self, parameters: &BTreeMap<String,String>, http_request_type: &str, uri: &str) -> String {
        //generate nonce and timestamp
        let nonce = rand::thread_rng().gen_ascii_chars().take(32).collect::<String>();
        let timestamp = time::now_utc().to_timespec().sec;

        //generate oauth_signature
        let mut map = BTreeMap::new();
        for (key, value) in parameters.iter() {
            map.insert(key.clone(), percent_encode(value.clone()));
        }

        map.insert("oauth_consumer_key".to_string(), self.consumer_key.clone());
        map.insert("oauth_nonce".to_string(), nonce.clone());
        map.insert("oauth_signature_method".to_string(), "HMAC-SHA1".to_string());
        map.insert("oauth_timestamp".to_string(), timestamp.to_string());
        map.insert("oauth_token".to_string(), self.access_token.clone());
        map.insert("oauth_version".to_string(), "1.0".to_string());

        let mut parameter_string = String::new();
        for (key, value) in map.iter() {
            parameter_string.push_str(&format!("&{}={}", key, value)[..]);
        }

        let signature_base_string = format!("{}&{}&{}", http_request_type, percent_encode(uri.to_string()), percent_encode(parameter_string[1..].to_string()));
        let signing_key = format!("{}&{}", percent_encode(self.consumer_secret.clone()), percent_encode(self.access_token_secret.clone()));

        let mut hmac = Hmac::new(Sha1::new(), &signing_key.into_bytes());
        hmac.input(&signature_base_string.into_bytes());
        let oauth_signature = hmac.result().code().to_base64(base64::STANDARD);       

        //return formatted authorization header
        format!("OAuth \
            oauth_consumer_key=\"{}\", \
            oauth_nonce=\"{}\", \
            oauth_signature=\"{}\", \
            oauth_signature_method=\"HMAC-SHA1\", \
            oauth_timestamp=\"{}\", \
            oauth_token=\"{}\", \
            oauth_version=\"1.0\"", 
            percent_encode(self.consumer_key.clone()),
            nonce,
            percent_encode(oauth_signature),
            timestamp,
            percent_encode(self.access_token.clone()),
        )
    }
}
