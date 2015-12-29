pub fn build_oauth_string(consumer_key: String, consumer_secret: String, access_token: String, access_token_secret: String) -> String {
    format!("OAuth \
        oauth_consumer_key=\"{}\", \
        oauth_nonce=\"TODO\", \
        oauth_signature=\"TODO\", \
        oauth_signature_method=\"HMAC-SHA1\", \
        oauth_timestamp=\"TODO\", \
        oauth_token=\"{}\", \
        oauth_version=\"1.0\"", 
        percent_encode(consumer_key),
        percent_encode(access_token),
    )
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
    fn test_build_oauth_string() {
        println!("{}", build_oauth_string("test_key".to_string(), "test_consumer_secret".to_string(), "test_token".to_string(), "test_token_secret".to_string()));
    }

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("Ladies + Gentleman".to_string()), "Ladies%20%2B%20Gentleman".to_string());
        assert_eq!(percent_encode("Dogs, Cats, & Mice!".to_string()), "Dogs%2C%20Cats%2C%20%26%20Mice%21".to_string());
    }
}
