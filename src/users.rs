use {RequestConfig,OAuthConfig};
use get_authorization_header;

use std::collections::BTreeMap;
use std::io::Read;

use hyper::Client;
use hyper::header::Authorization;
use hyper::status::StatusCode;

static URI: &'static str = "https://api.twitter.com/1.1/users/show.json";

pub fn create_show_config(user_id: Option<String>, screen_name: Option<String>) -> RequestConfig {
    let mut parameters = BTreeMap::new();

    if let Some(user_id) = user_id {
        parameters.insert("user_id".to_string(), user_id);
    }

    if let Some(screen_name) = screen_name {
        parameters.insert("screen_name".to_string(), screen_name);
    }

    RequestConfig {
        parameters: parameters,
    }
}

pub fn get_user_show(request_config: &RequestConfig, oauth_config: &OAuthConfig) -> Result<String,String> {
    let data_body = request_config.get_data_body();
    let authorization_header = get_authorization_header(request_config, oauth_config, URI);

    //send http post message
    let client = Client::new();
    let mut res = client.get(&format!("{}?{}", URI, data_body)[..])
        .header(Authorization(authorization_header))
        .send().unwrap();

    //read body
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    //check status code of http response
    if res.status != StatusCode::Ok {
        return Err(format!("http response has status code '{:?}' and body '{}'", res.status, body));
    }

    Ok(body)
}
