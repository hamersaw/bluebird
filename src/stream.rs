use std::collections::BTreeMap;
use std::sync::mpsc::{channel,Receiver};
use std::thread;

use oauth::OAuthConfig;
use percent_encode;

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use curl::http;
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

    pub fn get_data_string(&self) -> String {
        //TODO
        "".to_string()
    }

    pub fn get_oauth_string(&self) -> String {
        //TODO
        "".to_string()
    }

    pub fn get_oauth_signature(&self) -> String {
        let mut map = BTreeMap::new();

        if let Some(follow) = self.follow.clone() {
            map.insert("follow", follow);
        }

        if let Some(track) = self.track.clone() {
            map.insert("track", track);
        }

        if let Some(locations) = self.locations.clone() {
            map.insert("locations", locations);
        }

        if let Some(delimited) = self.delimited {
            map.insert("delimited", delimited.to_string());
        }

        if let Some(stall_warnings) = self.stall_warnings.clone() {
            map.insert("stall_warnings", stall_warnings.to_string());
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

        let mut hmac = Hmac::new(Sha1::new(), &signing_key.into_bytes());
        hmac.input(&parameter_string.into_bytes());
        hmac.result().code().to_base64(base64::STANDARD)
    }
}

pub fn open_filter_stream(filter_stream_config: &FilterStreamConfig) -> Receiver<String> {
    //TODO check for at least one filter

    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        let data_string = filter_stream_config.get_data_string();
        let oauth_string = filter_stream_config.get_oauth_string();

        let resp = http::handle()
            .post("https://stream.twitter.com/1.1/statuses/filter.json", &data_string[..])
            .header("Authorization", oauth_string.as_ref())
            .content_type("application/x-www-form-urlencoded")
            .exec().unwrap();

        if resp.get_code() != 200 {
            panic!("error with request");
        }

        //TODO read from stream and send tweets through channel
        tx.send("We have a successful stream opened".to_string()).unwrap();
    });

    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use oauth::OAuthConfig;

    #[test]
    fn test_generate_oauth_signature() {
        let oauth_config = OAuthConfig::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
        let filter_stream_config = FilterStreamConfig::new(None, Some("red,blue,yellow".to_string()), None, Some(true), Some(false), oauth_config);

        println!("signature:{}", filter_stream_config.get_oauth_signature());
    }
}
