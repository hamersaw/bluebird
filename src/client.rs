use oauth::OAuthConfig;
use request::{GetRequest,PostRequest,StreamRequest};

use std::collections::BTreeMap;

pub struct Client {
    oauth_config: OAuthConfig,
}

impl Client {
    /// Create a new client with the provided tokens and secrets
    ///
    /// # Examples
    /// ```
    /// let client = Client::new(
    ///     "consumer_key".to_string(),
    ///     "consumer_secret".to_string(),
    ///     "access_token".to_string(),
    ///     "access_token_secret".to_string()
    /// );
    /// ```
    pub fn new(consumer_token: String, consumer_secret: String, access_token: String, access_token_secret:String) -> Client {
        Client {
            oauth_config: OAuthConfig::new(consumer_token, consumer_secret, access_token, access_token_secret),
        }
    }

    /// Looks up a set of users based on screen names or user ids. `screen_name` is a comma
    /// separated list of twitter screen names. `user_id` is a comma separated list of integer
    /// user ids.
    ///
    /// # Examples
    /// ```
    /// match client.lookup_users(Some("twitterapi,twitter"), None) {
    ///     Ok(json) => println!("{}", json),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn lookup_users(&self, screen_name: Option<String>, user_id: Option<String>) -> PostRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();

        PostRequest::new("https://api.twitter.com/1.1/users/lookup.json", parameters, self.oauth_config.clone())
    }

    /// Searches a user based on screen name. `q` is the screen name to be searched. `page` is the
    /// page number of results to be returned (defaults to 1). `count` is the number of items per
    /// page (defaults to 20).
    ///
    /// # Examples
    /// ```
    /// match client.search_users(Some("twitterapi"), None, None) {
    ///     Ok(json) => println!("{}", json),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn search_users(&self, q: String, page: Option<String>, count: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("q".to_string(), q);
        insert_on_some("page", page, &mut parameters).unwrap();
        insert_on_some("count", count, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/users/search.json", parameters, self.oauth_config.clone())
    }

    /// Shows information for a single user based on screen names or user ids. `screen_name` is a
    /// screen name of a twitter account. `user_id` is the integer id of a twitter account.
    ///
    /// # Examples
    /// ```
    /// match client.show_users(Some("twitterapi"), None) {
    ///     Ok(json) => println!("{}", json),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn show_user(&self, user_id: Option<String>, screen_name: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/users/show.json", parameters, self.oauth_config.clone())
    }

    /// Retrieve tweets from your home timeline. `count` is the number of messages to return.
    /// `since_id` is the id of the oldest tweet to return. `max_id` is the id of the newest tweet
    /// to return.
    ///
    /// # Examples
    /// ```
    /// match client.home_timeline(Some("10".to_string()), None, None) {
    ///     Ok(json) => println!("{}", json),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn home_timeline(&self, count: Option<String>, since_id: Option<String>, max_id: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("count", count, &mut parameters).unwrap();
        insert_on_some("since_id", since_id, &mut parameters).unwrap();
        insert_on_some("max_id", max_id, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/statuses/home_timeline.json", parameters, self.oauth_config.clone())
    }

    /// Post a tweet. `status` is the tweet to be posted.
    ///
    /// # Examples
    /// ```
    /// match client.update_status("Testing the rust twitter api \"bluebird\" (https://github.com/hamersaw/bluebird)".to_string()).exec() {
    ///     Ok(json) => println!("{}", json),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn update_status(&self, status: String) -> PostRequest {
        let mut parameters = BTreeMap::new();
        parameters.insert("status".to_string(), status);

        PostRequest::new("https://api.twitter.com/1.1/statuses/update.json", parameters, self.oauth_config.clone())
    }

    /// Retrieve tweets from the specified users timeline. `count` is the number of messages to return.
    /// `since_id` is the id of the oldest tweet to return. `max_id` is the id of the newest tweet
    /// to return.
    ///
    /// # Examples
    /// ```
    /// match client.user_timeline(None, Some("twitterapi".to_string()), Some("10".to_string()), None, None).exec() {
    ///     Ok(json) => println!("{}", json),
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
    pub fn user_timeline(&self, user_id: Option<String>, screen_name: Option<String>, count: Option<String>, since_id: Option<String>, max_id: Option<String>) -> GetRequest {
        let mut parameters = BTreeMap::new();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();
        insert_on_some("count", count, &mut parameters).unwrap();
        insert_on_some("since_id", since_id, &mut parameters).unwrap();
        insert_on_some("max_id", max_id, &mut parameters).unwrap();

        GetRequest::new("https://api.twitter.com/1.1/statuses/user_timeline.json", parameters, self.oauth_config.clone())
    }

    /// Open a filter stream returning tweets that match criteria. `follow` is a comma separated
    /// list of screen names to follow. `track` is a comma separated list of keywords to track.
    /// `locations` is a comma separated list of coordinates to create a geographical bounding box. 
    ///    
    /// # Examples
    /// ```
    /// match client.open_filter_stream(None, Some("twitter".to_string()), None).exec() {
    ///     Ok(rx) => {
    ///         while let Ok(tweet) = rx.recv() {
    ///             println!("{}", tweet);
    ///         }
    ///     },
    ///     Err(e) => panic!("{}", e),
    /// }
    /// ```
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
