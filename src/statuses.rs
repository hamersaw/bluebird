use {RequestConfig,OAuthConfig};
use get_authorization_header;

use std::collections::BTreeMap;
use std::io::{BufRead,BufReader,Read};

use hyper::Client;
use hyper::header::{Authorization,ContentType};
use hyper::status::StatusCode;

static URI: &'static str = "https://api.twitter.com/1.1/statuses/update.json";

pub fn create_update_config(status: String) -> RequestConfig {
    let mut parameters = BTreeMap::new();
    parameters.insert("status".to_string(), status);

    RequestConfig {
        parameters: parameters,
    }
}

pub fn post_status_update(request_config: &RequestConfig, oauth_config: &OAuthConfig) -> Result<(),String> {
    let data_body = request_config.get_data_body();
    let authorization_header = get_authorization_header(request_config, oauth_config, URI);

    //send http post message
    let client = Client::new();
    let mut res = client.post(URI)
        .body(&data_body[..])
        .header(Authorization(authorization_header))
        .header(ContentType::form_url_encoded())
        .send().unwrap();

    //check status code of http response
    if res.status != StatusCode::Ok {
        let mut body = String::new();
        {
            let mut reader = BufReader::new(res.by_ref());
            reader.read_line(&mut body).unwrap();
        }

        return Err(format!("http response has status code '{:?}' and body '{}'", res.status, body));
    }

    Ok(())
}
