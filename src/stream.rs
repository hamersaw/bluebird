use RequestConfig;
use OAuthConfig;

use std::collections::HashMap;
use std::io::{BufRead,BufReader,Read};
use std::sync::mpsc::{channel,Receiver};
use std::thread;

use hyper::Client;
use hyper::header::{Authorization,ContentType};

pub fn create_filter_stream_config(follow: Option<String>, track: Option<String>, locations: Option<String>, oauth_config: OAuthConfig) -> RequestConfig {
    let mut parameters = HashMap::new();
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
        oauth_config: oauth_config,
    }
}

pub fn open_filter_stream(filter_stream_config: &RequestConfig) -> Result<Receiver<String>,String> {
    if filter_stream_config.get_parameter_count() < 2 { //we're automatically adding the delimited parameter
        return Err(format!("Need to specify at least one filter parameter to open a filter stream. Only {} was supplied", filter_stream_config.get_parameter_count() - 1));
    }

    let data_body = filter_stream_config.get_data_body();
    let authorization_header = filter_stream_config.get_authorization_header();
    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        let client = Client::new();
        let mut res = client.post("https://stream.twitter.com/1.1/statuses/filter.json")
            .body(&data_body[..])
            .header(Authorization(authorization_header))
            .header(ContentType::form_url_encoded())
            .send().unwrap();

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
