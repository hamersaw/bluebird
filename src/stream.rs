use std::sync::mpsc::{channel,Receiver};

use oauth::OAuthConfig;
use percent_encode;

pub struct FilterStreamConfig {
    follow: Option<String>, //comma separated list of users to follow
    track: Option<String>, //comma separated list of keywords to track
    locations: Option<String>, //specifies a set of bounding boxes to track
    delimited: bool, //specifies whether messages should be length delimited
    stall_warnings: bool, //specifies whether stall warnings should be delivered
    oauth_config: OAuthConfig
}

impl FilterStreamConfig {
    pub fn new(follow: Option<String>, track: Option<String>, locations: Option<String>, delimited: bool, stall_warnings: bool, oauth_config: OAuthConfig) -> FilterStreamConfig {
        FilterStreamConfig {
            follow: follow,
            track: track,
            locations: locations,
            delimited: delimited,
            stall_warnings: stall_warnings,
            oauth_config: oauth_config,
        }
    }

    pub fn generate_oauth_signature() -> String {
        "".to_string()
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
        let oauth_config = OAuthConfig::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
        let filter_stream_config = FilterStreamConfig::new(None, None, None, true, false, oauth_config);

        println!("{}", filter_stream_config.delimited);
    }

    #[test]
    fn test_generate_oauth_signature() {
        let oauth_config = OAuthConfig::new("".to_string(), "".to_string(), "".to_string(), "".to_string());
        let filter_stream_config = FilterStreamConfig::new(None, Some("red,blue,yellow".to_string()), None, true, false, oauth_config);

    }
}
