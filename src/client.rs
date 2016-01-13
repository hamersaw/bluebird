use oauth::OAuthConfig;
use request::{GetRequest,PostRequest,StreamRequest};

use std::collections::BTreeMap;

pub struct Client {
    oauth_config: OAuthConfig,
}

impl Client {
    pub fn new(consumer_token: String, consumer_secret: String, access_token: String, access_token_secret:String) -> Client {
        Client {
            oauth_config: OAuthConfig::new(consumer_token, consumer_secret, access_token, access_token_secret),
        }
    }

    //USERS
    pub fn show_user(&self, user_id: Option<String>, screen_name: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        insert_if_exists("user_id", user_id, &mut parameters).unwrap();
        insert_if_exists("screen_name", screen_name, &mut parameters).unwrap();

        GetRequest::new(
            "https://api.twitter.com/1.1/users/show.json",
            parameters,
            self.oauth_config.clone(),
        )
    }

    //STATUSES
    pub fn update_status(&self, status: String) -> PostRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("status".to_string(), status);

        PostRequest::new(
            "https://api.twitter.com/1.1/statuses/update.json",
            parameters,
            self.oauth_config.clone(),
        )
    }

    //STREAMS
    pub fn open_filter_stream(&self, follow: Option<String>, track: Option<String>, locations: Option<String>) -> StreamRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("delimited".to_string(), "length".to_string());
        insert_if_exists("follow", follow, &mut parameters).unwrap();
        insert_if_exists("track", track, &mut parameters).unwrap();
        insert_if_exists("locations", locations, &mut parameters).unwrap();

        StreamRequest::new(
            "https://stream.twitter.com/1.1/statuses/filter.json",
            parameters,
            self.oauth_config.clone(),
        )
    }
}

fn insert_if_exists(key: &str, value: Option<String>, map: &mut BTreeMap<String,String>) -> Result<(),String> {
    if let Some(value) = value {
        map.insert(key.to_string(), value);
    }

    Ok(())
}
