extern crate crypto;
extern crate hyper;
extern crate rand;
extern crate rustc_serialize;
extern crate time;

pub mod request;
use request::{HttpMethod,HttpRequest};

use std::collections::BTreeMap;
use std::io::{BufRead,BufReader,Read};
use std::sync::mpsc::{channel,Receiver};
use std::thread;

pub struct Client<'a> {
    consumer_key: &'a str,
    consumer_secret: &'a str,
    access_token: &'a str,
    access_token_secret: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(consumer_key: &'a str, consumer_secret: &'a str, access_token: &'a str, access_token_secret: &'a str) -> Client<'a> {
        Client {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            access_token: access_token,
            access_token_secret: access_token_secret,
        }
    }

    pub fn home_timeline(&self, count: Option<&'a str>, since_id: Option<&'a str>, max_id: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        multiple_insert(&mut parameters, &[("count", count), ("since_id", since_id), ("max_id", max_id)]).unwrap();

        self.send_http_request("https://api.twitter.com/1.1/statuses/home_timeline.json", HttpMethod::Get, parameters)
    }

    pub fn lookup_users(&self, screen_name: Option<&'a str>, user_id: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        multiple_insert(&mut parameters, &[("screen_name", screen_name), ("user_id", user_id)]).unwrap();

        self.send_http_request("https://api.twitter.com/1.1/users/lookup.json", HttpMethod::Post, parameters)
    }

    pub fn open_filter_stream(&self, follow: Option<&'a str>, track: Option<&'a str>, locations: Option<&'a str>) -> Result<Receiver<String>,String> {
        let mut parameters = BTreeMap::new();
        multiple_insert(&mut parameters, &[("delimited", Some("length")), ("follow", follow), ("track", track), ("locations", locations)]).unwrap();

        let http_request = HttpRequest::new(
            "https://stream.twitter.com/1.1/statuses/filter.json",
            HttpMethod::Post,
            parameters,
            self.consumer_key, self.consumer_secret, self.access_token, self.access_token_secret);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to send statuses filter http request");

        if status_code != 200 {
            let mut body = String::new();
            reader.read_to_string(&mut body).unwrap();
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }

        let (tx, rx) = channel::<String>();
        thread::spawn(move || {
            let mut buffer = String::new();
            let mut reader = BufReader::new(reader.by_ref());

            loop {
                //read number of bytes in tweet
                loop {
                    if reader.read_line(&mut buffer).ok().expect("unable to read number of bytes from filter stream") != 0 {
                        break;
                    }
                }

                //parse string into unsigned 32 bit integer
                let mut remaining_bytes = buffer.trim().parse::<u32>().unwrap();
                buffer.clear();

                //read tweet bytes
                let mut tweet = String::new();
                while remaining_bytes > 0 {
                    let bytes = reader.read_line(&mut tweet).ok().expect("unable to read tweet from filter stream");
                    remaining_bytes -= bytes as u32;
                }

                tx.send(tweet).unwrap();
            }
        });

        Ok(rx)
    }

    pub fn search_users(&self, q: &'a str, page: Option<&'a str>, count: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        multiple_insert(&mut parameters, &[("q", Some(q)), ("page", page), ("count", count)]).unwrap();

        self.send_http_request("https://api.twitter.com/1.1/users/search.json", HttpMethod::Get, parameters)
    }


    pub fn show_user(&self, screen_name: Option<&'a str>, user_id: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        multiple_insert(&mut parameters, &[("screen_name", screen_name), ("user_id", user_id)]).unwrap();

        self.send_http_request("https://api.twitter.com/1.1/users/show.json", HttpMethod::Get, parameters)
    }

    pub fn update_status(&self, status: &'a str) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("status", status);

        self.send_http_request("https://api.twitter.com/1.1/statuses/update.json", HttpMethod::Post, parameters)
    }

    pub fn user_timeline(&self, screen_name: Option<&'a str>, user_id: Option<&'a str>, count: Option<&'a str>, since_id: Option<&'a str>, max_id: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        multiple_insert(&mut parameters, &[("screen_name", screen_name), ("user_id", user_id), ("count", count), ("since_id", since_id), ("max_id", max_id)]).unwrap();

        self.send_http_request("https://api.twitter.com/1.1/statuses/user_timeline.json", HttpMethod::Get, parameters)
    }

    fn send_http_request(&self, uri: &'a str, method: HttpMethod, parameters: BTreeMap<&'a str,&'a str>) -> Result<String,String> {
        let http_request = HttpRequest::new(uri, method, parameters, self.consumer_key, self.consumer_secret, self.access_token, self.access_token_secret);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to http request from bluebird client");
        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();
        drop(reader);

        if status_code != 200 {
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }
        Ok(body)
    }
}

fn multiple_insert<'a>(map: &mut BTreeMap<&'a str,&'a str>, key_value_pairs: &[(&'a str, Option<&'a str>)]) -> Result<(),String> {
    for entry in key_value_pairs {
        if let Some(value) = entry.1 {
            map.insert(entry.0, value);
        }
    }

    Ok(())
}

#[test]
fn it_works() {
}
