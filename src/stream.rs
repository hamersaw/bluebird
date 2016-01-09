use std::collections::BTreeMap;
use std::io::{BufRead,BufReader,Read};
use std::sync::mpsc::{channel,Receiver};
use std::thread;

use oauth::OAuthConfig;
use percent_encode;

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use hyper::Client;
use hyper::header::{Authorization,ContentType};
use rustc_serialize::base64::{self,ToBase64};

pub struct FilterStreamConfig {
    follow: Option<String>, //comma separated list of users to follow
    track: Option<String>, //comma separated list of keywords to track
    locations: Option<String>, //specifies a set of bounding boxes to track
    oauth_config: OAuthConfig
}

impl FilterStreamConfig {
    pub fn new(follow: Option<String>, track: Option<String>, locations: Option<String>, oauth_config: OAuthConfig) -> FilterStreamConfig {
        FilterStreamConfig {
            follow: follow,
            track: track,
            locations: locations,
            oauth_config: oauth_config,
        }
    }

    pub fn get_data_string(&self) -> String {
        let mut data_string = "delimited=length".to_string();

        if let Some(follow) = self.follow.clone() {
            data_string.push_str(&format!("&format={}", follow)[..]);
        }

        if let Some(track) = self.track.clone() {
            data_string.push_str(&format!("&track={}", track)[..]);
        }

        if let Some(locations) = self.locations.clone() {
            data_string.push_str(&format!("&locations={}", locations)[..]);
        }

        data_string
    }

    pub fn get_oauth_string(&self) -> String {
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

    pub fn get_oauth_signature(&self) -> String {
        let mut map = BTreeMap::new();
        map.insert("delimited", "length".to_string());

        if let Some(follow) = self.follow.clone() {
            map.insert("follow", follow);
        }

        if let Some(track) = self.track.clone() {
            map.insert("track", track);
        }

        if let Some(locations) = self.locations.clone() {
            map.insert("locations", locations);
        }

        map.insert("oauth_consumer_key", self.oauth_config.consumer_key.clone());
        map.insert("oauth_nonce", self.oauth_config.nonce.clone());
        map.insert("oauth_signature_method", "HMAC-SHA1".to_string());
        map.insert("oauth_timestamp", self.oauth_config.timestamp.to_string());
        map.insert("oauth_token", self.oauth_config.access_token.clone());
        map.insert("oauth_version", "1.0".to_string());

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
}

pub fn open_filter_stream(filter_stream_config: &FilterStreamConfig) -> Receiver<String> {
    //TODO check for at least one filter

    let data_string = filter_stream_config.get_data_string();
    let oauth_string = filter_stream_config.get_oauth_string();
    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        let client = Client::new();
        let mut res = client.post("https://stream.twitter.com/1.1/statuses/filter.json")
            .body(&data_string[..])
            .header(Authorization(oauth_string))
            .header(ContentType::form_url_encoded())
            .send().unwrap();

        let mut buffer = String::new();
        let mut reader = BufReader::new(res.by_ref());
        loop {
            //read number of bytes in tweet
            loop {
                match reader.read_line(&mut buffer) {
                    Ok(bytes) => {
                        if bytes != 0 {
                            break;
                        }
                    },
                    Err(e) => panic!("{}", e),
                }
            }

            //parse string into unsigned 32 bit integer
            let mut remaining_bytes = buffer.trim().parse::<u32>().unwrap();
            buffer.clear();

            //read tweet bytes
            let mut tweet = String::new();
            while remaining_bytes > 0 {
                match reader.read_line(&mut tweet) {
                    Ok(bytes) => remaining_bytes -= bytes as u32,
                    Err(e) => panic!("{}", e),
                }
            }

            tx.send(tweet).unwrap();
        }
    });

    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use oauth::OAuthConfig;

    #[test]
    fn test_get_data_string() {
        let oauth_config = OAuthConfig::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
        let filter_stream_config = FilterStreamConfig::new(None, Some("red,blue,yellow".to_string()), None, oauth_config);

        println!("data_string:\"{}\"", filter_stream_config.get_data_string());
    }

    #[test]
    fn test_get_oauth_string() {
        let oauth_config = OAuthConfig::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
        let filter_stream_config = FilterStreamConfig::new(None, Some("red,blue,yellow".to_string()), None, oauth_config);

        println!("oauth_string:\"{}\"", filter_stream_config.get_oauth_string());
    }

    #[test]
    fn test_get_oauth_signature() {
        let oauth_config = OAuthConfig::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
        let filter_stream_config = FilterStreamConfig::new(None, Some("red,blue,yellow".to_string()), None, oauth_config);

        println!("signature:{}", filter_stream_config.get_oauth_signature());
    }
}
