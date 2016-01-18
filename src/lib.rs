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
        insert_on_some("count", count, &mut parameters).unwrap();
        insert_on_some("since_id", since_id, &mut parameters).unwrap();
        insert_on_some("max_id", max_id, &mut parameters).unwrap();

        let http_request = self.new_http_request("https://api.twitter.com/1.1/statuses/home_timeline.json", HttpMethod::Get, parameters);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to send home timeline statuses http request");
        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();

        if status_code != 200 {
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }
        Ok(body)
    }

    pub fn lookup_users(&self, screen_name: Option<&'a str>, user_id: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();

        let http_request = self.new_http_request("https://api.twitter.com/1.1/users/lookup.json", HttpMethod::Post, parameters);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to send lookup user http request");
        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();

        if status_code != 200 {
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }
        Ok(body)
    }

    pub fn open_filter_stream(&self, follow: Option<&'a str>, track: Option<&'a str>, locations: Option<&'a str>) -> Result<Receiver<String>,String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("delimited", "length");
        insert_on_some("follow", follow, &mut parameters).unwrap();
        insert_on_some("track", track, &mut parameters).unwrap();
        insert_on_some("locations", locations, &mut parameters).unwrap();

        let http_request = self.new_http_request("https://stream.twitter.com/1.1/statuses/filter.json", HttpMethod::Post, parameters);
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
        parameters.insert("q", q);
        insert_on_some("page", page, &mut parameters).unwrap();
        insert_on_some("count", count, &mut parameters).unwrap();

        let http_request = self.new_http_request("https://api.twitter.com/1.1/users/search.json", HttpMethod::Get, parameters);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to send search user http request");
        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();

        if status_code != 200 {
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }
        Ok(body)
    }


    pub fn show_user(&self, screen_name: Option<&'a str>, user_id: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();

        let http_request = self.new_http_request("https://api.twitter.com/1.1/users/show.json", HttpMethod::Get, parameters);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to send show user http request");
        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();

        if status_code != 200 {
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }
        Ok(body)
    }

    pub fn update_status(&self, status: &'a str) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("status", status);

        let http_request = self.new_http_request("https://api.twitter.com/1.1/statuses/update.json", HttpMethod::Post, parameters);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to send update status http request");
        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();

        if status_code != 200 {
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }
        Ok(body)
    }

    pub fn user_timeline(&self, screen_name: Option<&'a str>, user_id: Option<&'a str>, count: Option<&'a str>, since_id: Option<&'a str>, max_id: Option<&'a str>) -> Result<String,String> {
        let mut parameters = BTreeMap::new();
        insert_on_some("screen_name", screen_name, &mut parameters).unwrap();
        insert_on_some("user_id", user_id, &mut parameters).unwrap();
        insert_on_some("count", count, &mut parameters).unwrap();
        insert_on_some("since_id", since_id, &mut parameters).unwrap();
        insert_on_some("max_id", max_id, &mut parameters).unwrap();

        let http_request = self.new_http_request("https://api.twitter.com/1.1/statuses/user_timeline.json", HttpMethod::Get, parameters);
        let (mut reader, status_code) = http_request.send().ok().expect("failed to send user timeline statuses http request");
        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();

        if status_code != 200 {
            return Err(format!("HTTP status code '{}' and body '{}'", status_code, body));
        }
        Ok(body)
    }

    fn new_http_request(&self, uri: &'a str, method: HttpMethod, parameters: BTreeMap<&'a str,&'a str>) -> HttpRequest {
        HttpRequest::new(uri, method, parameters, self.consumer_key, self.consumer_secret, self.access_token, self.access_token_secret)
    }
}

fn insert_on_some<'a>(key: &'a str, value: Option<&'a str>, parameters: &mut BTreeMap<&'a str,&'a str>) -> Result<(),String> {
    if let Some(value) = value {
        parameters.insert(key, value);
    }

    Ok(())
}

#[test]
fn it_works() {
}
