use std::collections::BTreeMap;
use std::sync::mpsc::{channel,Receiver};

use oauth::OAuthConfig;
use percent_encode;

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use rustc_serialize::base64::{self,ToBase64};

pub struct FilterStreamConfig {
    follow: Option<String>, //comma separated list of users to follow
    track: Option<String>, //comma separated list of keywords to track
    locations: Option<String>, //specifies a set of bounding boxes to track
    delimited: Option<bool>, //specifies whether messages should be length delimited
    stall_warnings: Option<bool>, //specifies whether stall warnings should be delivered
    oauth_config: OAuthConfig
}

impl FilterStreamConfig {
    pub fn new(follow: Option<String>, track: Option<String>, locations: Option<String>, delimited: Option<bool>, stall_warnings: Option<bool>, oauth_config: OAuthConfig) -> FilterStreamConfig {
        FilterStreamConfig {
            follow: follow,
            track: track,
            locations: locations,
            delimited: delimited,
            stall_warnings: stall_warnings,
            oauth_config: oauth_config,
        }
    }

    pub fn generate_oauth_signature(&self) -> String {
        let mut map = BTreeMap::new();

        match self.follow.clone() {
            Some(follow) => { map.insert("follow", follow); },
            _ => {},
        }

        match self.track.clone() {
            Some(track) => { map.insert("track", track); },
            _ => {},
        }

        match self.locations.clone() {
            Some(locations) => { map.insert("locations", locations); },
            _ => {},
        }

        match self.delimited {
            Some(delimited) => { map.insert("delimited", delimited.to_string()); },
            _ => {},
        }

        match self.stall_warnings {
            Some(stall_warnings) => { map.insert("stall_warnings", stall_warnings.to_string()); },
            _ => {},
        }

        map.insert("oauth_consumer_key", self.oauth_config.consumer_key.clone());
        map.insert("oauth_nonce", self.oauth_config.nonce.clone());
        map.insert("oauth_signature_method", "HMAC-SHA1".to_string());
        map.insert("oauth_timestamp", self.oauth_config.timestamp.to_string());
        map.insert("oauth_token", self.oauth_config.access_token.clone());
        map.insert("oauth_version", "1.0".to_string());

        let mut signature_base_string = String::new();
        for (key, value) in map.iter() {
            signature_base_string.push_str(&format!("&{}={}", key, value)[..]);
        }

        let parameter_string = percent_encode(format!("POST&https://stream.twitter.com/1.1/statuses/filter.json{}", signature_base_string));
        let signing_key = format!("{}&{}", percent_encode(self.oauth_config.consumer_secret.clone()), percent_encode(self.oauth_config.access_token_secret.clone()));

        let mut hmac = Hmac::new(Sha1::new(), &[0u8; 0]);
        hmac.input(&parameter_string.into_bytes());
        hmac.input(&signing_key.into_bytes());

        hmac.result().code().to_base64(base64::STANDARD)
    }
}

pub fn open_filter_stream(filter_stream_config: &FilterStreamConfig) -> Receiver<String> {
    let (tx, rx) = channel::<String>();

    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use oauth::OAuthConfig;

    #[test]
    fn test_open_filter_stream() {
    }

    #[test]
    fn test_generate_oauth_signature() {
        let oauth_config = OAuthConfig::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
        let filter_stream_config = FilterStreamConfig::new(None, Some("red,blue,yellow".to_string()), None, Some(true), Some(false), oauth_config);

        println!("signature:{}", filter_stream_config.generate_oauth_signature());
    }
}
