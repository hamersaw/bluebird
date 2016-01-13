use percent_encode;
use oauth::OAuthConfig;

use std::collections::BTreeMap;
use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization,ContentType};
use hyper::status::StatusCode;

pub trait BluebirdRequest {
    fn exec(&self) -> Result<String,String>;
}

//GET REQUEST
pub struct GetRequest<'a> {
    uri: &'a str,
    parameters: BTreeMap<String,String>,
    oauth_config: OAuthConfig,
}

impl<'a> GetRequest<'a> {
    pub fn new(uri: &str, parameters: BTreeMap<String,String>, oauth_config: OAuthConfig) -> GetRequest {
        GetRequest {
            uri: uri,
            parameters: parameters,
            oauth_config: oauth_config,
        }
    }
}

impl<'a> BluebirdRequest for GetRequest<'a> {
    fn exec(&self) -> Result<String,String> {
        let data_string = get_data_string(&self.parameters);
        let authorization_header = self.oauth_config.get_authorization_header(&self.parameters, "GET", self.uri);

        //send http get message
        let client = Client::new();
        let mut res = client.get(&format!("{}?{}", self.uri, data_string)[..])
            .header(Authorization(authorization_header))
            .send().unwrap();

        //read body
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        //check status code of http response
        if res.status != StatusCode::Ok {
            return Err(format!("http response has code '{:?}' and body '{}'", res.status, body));
        }

        Ok(body)
    }
}

//POST REQUEST
pub struct PostRequest<'a> {
    uri: &'a str,
    parameters: BTreeMap<String,String>,
    oauth_config: OAuthConfig,
}

impl<'a> PostRequest<'a> {
    pub fn new(uri: &str, parameters: BTreeMap<String,String>, oauth_config: OAuthConfig) -> PostRequest {
        PostRequest {
            uri: uri,
            parameters: parameters,
            oauth_config: oauth_config,
        }
    }
}

impl<'a> BluebirdRequest for PostRequest<'a> {
    fn exec(&self) -> Result<String,String> {
        let data_string = get_data_string(&self.parameters);
        let authorization_header = self.oauth_config.get_authorization_header(&self.parameters, "POST", self.uri);

        //send http post message
        let client = Client::new();
        let mut res = client.post(self.uri)
            .header(Authorization(authorization_header))
            .header(ContentType::form_url_encoded())
            .body(&data_string[..])
            .send().unwrap();

        //read body
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        //check status code of http response
        if res.status != StatusCode::Ok {
            return Err(format!("http response has code '{:?}' and body '{}'", res.status, body));
        }

        Ok(body)
    }
}

fn get_data_string(parameters: &BTreeMap<String,String>) -> String {
    let mut data_string = String::new();
    for (i, (key, value)) in parameters.iter().enumerate() {
        if i != 0 {
            data_string.push_str("&");
        }

        data_string.push_str(&format!("{}={}", key, percent_encode(value.clone()))[..]);
    }

    data_string
}
