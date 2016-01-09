use {RequestConfig,OAuthConfig};
use get_authorization_header;

use std::collections::HashMap;

use hyper::Client;
use hyper::header::{Authorization,ContentType};
use hyper::status::StatusCode;

static URI: &'static str = "https://api.twitter.com/1.1/statuses/update.json";

pub fn create_update_config(status: String) -> RequestConfig {
    let mut parameters = HashMap::new();
    parameters.insert("status".to_string(), status);

    RequestConfig {
        parameters: parameters,
    }
}

pub fn post_status_update(request_config: &RequestConfig, oauth_config: &OAuthConfig) -> Result<(),String> {
    let data_body = request_config.get_data_body();
    let authorization_header = get_authorization_header(request_config, oauth_config, URI);
    
    let client = Client::new();
    let res = client.post(URI)
        .body(&data_body[..])
        .header(Authorization(authorization_header))
        .header(ContentType::form_url_encoded())
        .send().unwrap();

    if res.status != StatusCode::Ok {
        return Err(format!("status update returned code {:?}", res.status));
    }

    Ok(())
}
