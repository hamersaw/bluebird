use percent_encode;
use oauth::OAuthConfig;

use std::collections::BTreeMap;
use std::io::{BufRead,BufReader,Read};
use std::sync::mpsc::{channel,Receiver};
use std::thread;

use hyper::Client;
use hyper::header::{Authorization,ContentType};
use hyper::status::StatusCode;

/// Trait requiring an exec() function for bluebird request structs
pub trait BluebirdRequest<E> {
    fn exec(&self) -> Result<E,String>;
}

/// Struct representing an HTTP GET request
pub struct GetRequest<'a> {
    uri: &'a str,
    parameters: BTreeMap<String,String>,
    oauth_config: OAuthConfig,
}

impl<'a> GetRequest<'a> {
    /// Creates a new GetRequest struct
    pub fn new(uri: &str, parameters: BTreeMap<String,String>, oauth_config: OAuthConfig) -> GetRequest {
        GetRequest {
            uri: uri,
            parameters: parameters,
            oauth_config: oauth_config,
        }
    }
}

impl<'a> BluebirdRequest<String> for GetRequest<'a> {
    /// Executes the GetRequest returning the json response from the twitter REST API.
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

/// Struct representing an HTTP POST request
pub struct PostRequest<'a> {
    uri: &'a str,
    parameters: BTreeMap<String,String>,
    oauth_config: OAuthConfig,
}

impl<'a> PostRequest<'a> {
    /// Creates a new PostRequest struct
    pub fn new(uri: &str, parameters: BTreeMap<String,String>, oauth_config: OAuthConfig) -> PostRequest {
        PostRequest {
            uri: uri,
            parameters: parameters,
            oauth_config: oauth_config,
        }
    }
}

impl<'a> BluebirdRequest<String> for PostRequest<'a> {
    /// Executes the PostRequest returning the json response from the twitter REST API.
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

/// Struct used to issue a filter stream request
pub struct StreamRequest<'a> {
    uri: &'a str,
    parameters: BTreeMap<String,String>,
    oauth_config: OAuthConfig,
}

impl<'a> StreamRequest<'a> {
    /// Creates a new StreamRequest struct
    pub fn new(uri: &str, parameters: BTreeMap<String,String>, oauth_config: OAuthConfig) -> StreamRequest {
        StreamRequest {
            uri: uri,
            parameters: parameters,
            oauth_config: oauth_config,
        }
    }
}

impl<'a> BluebirdRequest<Receiver<String>> for StreamRequest<'a> {
    /// Executes the StreamRequest returning a channel that recieves individual tweets (in json)
    /// that are returned from the twitter REST API.
    fn exec(&self) -> Result<Receiver<String>,String> {
        let data_string = get_data_string(&self.parameters);
        let authorization_header = self.oauth_config.get_authorization_header(&self.parameters, "POST", self.uri);

        //send http post message
        let client = Client::new();
        let mut res = client.post(self.uri)
            .header(Authorization(authorization_header))
            .header(ContentType::form_url_encoded())
            .body(&data_string[..])
            .send().unwrap();

        //check status code of http response
        if res.status != StatusCode::Ok {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();

            return Err(format!("http response has code '{:?}' and body '{}'", res.status, body));
        }

        let (tx, rx) = channel::<String>();
        thread::spawn(move || {
            let mut buffer = String::new();
            let mut reader = BufReader::new(res.by_ref());

            loop {
                //read number of bytes in tweet
                loop {
                    match reader.read_line(&mut buffer) {
                        Ok(bytes) => {
                            if bytes != 0 {
                                break;
                            }
                        },
                        Err(e) => panic!("{}", e),
                    }
                }

                //parse string into unsigned 32 bit integer
                let mut remaining_bytes = buffer.trim().parse::<u32>().unwrap();
                buffer.clear();

                //read tweet bytes
                let mut tweet = String::new();
                while remaining_bytes > 0 {
                    match reader.read_line(&mut tweet) {
                        Ok(bytes) => remaining_bytes -= bytes as u32,
                        Err(e) => panic!("{}", e),
                    }
                }

                tx.send(tweet).unwrap();
            }
        });

        Ok(rx)
    }
}

/// Returns a '&' delimited list of key value pairs for entries in a BTreeMap.
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
