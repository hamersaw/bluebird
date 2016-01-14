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
    pub fn lookup_users(&self, screen_name: Option<String>, user_id: Option<String>) -> PostRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();

        PostRequest::new("https://api.twitter.com/1.1/users/lookup.json", parameters, self.oauth_config.clone())
    }

    pub fn search_users(&self, q: String, page: Option<String>, count: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("q".to_string(), q);
        insert_on_some("page", page, &mut parameters).unwrap();
        insert_on_some("count", count, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/users/search.json", parameters, self.oauth_config.clone())
    }

    pub fn show_user(&self, user_id: Option<String>, screen_name: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/users/show.json", parameters, self.oauth_config.clone())
    }

    //STATUSES
    pub fn home_timeline(&self, count: Option<String>, since_id: Option<String>, max_id: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("count", count, &mut parameters).unwrap();
        insert_on_some("since_id", since_id, &mut parameters).unwrap();
        insert_on_some("max_id", max_id, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/statuses/home_timeline.json", parameters, self.oauth_config.clone())
    }

    pub fn update_status(&self, status: String) -> PostRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("status".to_string(), status);

        PostRequest::new("https://api.twitter.com/1.1/statuses/update.json", parameters, self.oauth_config.clone())
    }

    pub fn user_timeline(&self, user_id: Option<String>, screen_name: Option<String>, count: Option<String>, since_id: Option<String>, max_id: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();
        insert_on_some("count", count, &mut parameters).unwrap();
        insert_on_some("since_id", since_id, &mut parameters).unwrap();
        insert_on_some("max_id", max_id, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/statuses/user_timeline.json", parameters, self.oauth_config.clone())
    }

    //STREAMS
    pub fn open_filter_stream(&self, follow: Option<String>, track: Option<String>, locations: Option<String>) -> StreamRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("delimited".to_string(), "length".to_string());
        insert_on_some("follow", follow, &mut parameters).unwrap();
        insert_on_some("track", track, &mut parameters).unwrap();
        insert_on_some("locations", locations, &mut parameters).unwrap();

        StreamRequest::new("https://stream.twitter.com/1.1/statuses/filter.json", parameters, self.oauth_config.clone())
    }
}

fn insert_on_some(key: &str, value: Option<String>, map: &mut BTreeMap<String,String>) -> Result<(),String> {
    if let Some(value) = value {
        map.insert(key.to_string(), value);
    }

    Ok(())
}
