use {RequestConfig,OAuthConfig};
use get_authorization_header;

use std::collections::BTreeMap;
use std::io::{BufRead,BufReader,Read};
use std::sync::mpsc::{channel,Receiver};
use std::thread;

use hyper::Client;
use hyper::header::{Authorization,ContentType};
use hyper::status::StatusCode;

static URI: &'static str = "https://stream.twitter.com/1.1/statuses/filter.json";

pub fn create_stream_config(follow: Option<String>, track: Option<String>, locations: Option<String>) -> RequestConfig {
    let mut parameters = BTreeMap::new();
    parameters.insert("delimited".to_string(), "length".to_string());

    if let Some(follow) = follow {
        parameters.insert("follow".to_string(), follow);
    }

    if let Some(track) = track {
        parameters.insert("track".to_string(), track);
    }

    if let Some(locations) = locations {
        parameters.insert("locations".to_string(), locations);
    }

    RequestConfig {
        parameters: parameters,
    }
}

pub fn open_stream(request_config: &RequestConfig, oauth_config: &OAuthConfig) -> Result<Receiver<String>,String> {
    if request_config.get_parameter_count() < 2 { //we're automatically adding the delimited parameter
        return Err(format!("Need to specify at least one filter parameter to open a filter stream. Only {} was supplied", request_config.get_parameter_count() - 1));
    }

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
