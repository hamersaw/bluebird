use oauth::OAuthConfig;
use request::{GetRequest,PostRequest};

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

    pub fn show_user(&self, user_id: Option<String>, screen_name: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        
        if let Some(user_id) = user_id {
            parameters.insert("user_id".to_string(), user_id);
        }

        if let Some(screen_name) = screen_name {
            parameters.insert("screen_name".to_string(), screen_name);
        }

        GetRequest::new(
            "https://api.twitter.com/1.1/users/show.json",
            parameters,
            self.oauth_config.clone(),
        )
    }

    pub fn update_status(&self, status: String) -> PostRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("status".to_string(), status);

        PostRequest::new(
            "https://api.twitter.com/1.1/statuses/update.json",
            parameters,
            self.oauth_config.clone(),
        )
    }
}
