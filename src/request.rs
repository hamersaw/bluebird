use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use hyper::Client;
use hyper::header::{Authorization,ContentType};
use rand::{self,Rng};
use rustc_serialize::base64::{self,ToBase64};
use time;

use std::collections::BTreeMap;
use std::io::Read;

pub enum HttpMethod {
    Get,
    Head,
    Post,
}

pub struct HttpRequest<'a> {
    uri: &'a str,
    method: HttpMethod,
    parameters: BTreeMap<&'a str,&'a str>,
    nonce: String,
    timestamp: i64,
    consumer_key: &'a str,
    consumer_secret: &'a str,
    access_token: &'a str,
    access_token_secret: &'a str,
}

impl<'a> HttpRequest<'a> {
    pub fn new(uri: &'a str, method: HttpMethod, parameters: BTreeMap<&'a str,&'a str>, consumer_key: &'a str, consumer_secret: &'a str, access_token: &'a str, access_token_secret: &'a str) -> HttpRequest<'a> {
        HttpRequest {
            uri: uri,
            method: method,
            parameters: parameters,
            nonce: rand::thread_rng().gen_ascii_chars().take(32).collect::<String>(),
            timestamp: time::now_utc().to_timespec().sec,
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            access_token: access_token,
            access_token_secret: access_token_secret,
        }
    }

    pub fn send(&self) -> Result<(Box<Read+Send>,u16),String> {
        let body = self.get_body();
        let authorization_header = self.get_authorization_header();

        let client = Client::new();
        let res = match self.method {
            HttpMethod::Get => {
                client.get(&format!("{}?{}", self.uri, body))
                    .header(Authorization(authorization_header))
                    .send().unwrap()
            },
            HttpMethod::Head => {
                unimplemented!()
            },
            HttpMethod::Post => {
                client.post(self.uri)
                    .header(Authorization(authorization_header))
                    .header(ContentType::form_url_encoded())
                    .body(&body)
                    .send().unwrap()
            }
        };

        let status_code = res.status_raw().0;
        Ok((Box::new(res), status_code))
    }

    fn get_body(&self) -> String {
        let mut data_string = String::new();
        for (i, (key, value)) in self.parameters.iter().enumerate() {
            if i != 0 {
                data_string.push_str("&");
            }

            data_string.push_str(&format!("{}={}", key, percent_encode(value))[..]);
        }

        data_string
    }

    fn get_authorization_header(&self) -> String {
        format!("OAuth \
            oauth_consumer_key=\"{}\", \
            oauth_nonce=\"{}\", \
            oauth_signature=\"{}\", \
            oauth_signature_method=\"HMAC-SHA1\", \
            oauth_timestamp=\"{}\", \
            oauth_token=\"{}\", \
            oauth_version=\"1.0\"", 
            percent_encode(self.consumer_key),
            self.nonce,
            percent_encode(&self.get_oauth_signature()),
            self.timestamp,
            percent_encode(self.access_token),
        )
    }

    fn get_oauth_signature(&self) -> String {
        let mut map = BTreeMap::new();
        for (key, value) in self.parameters.iter() {
            map.insert(*key, percent_encode(value));
        }

        map.insert("oauth_consumer_key", self.consumer_key.to_string());
        map.insert("oauth_nonce", self.nonce.clone());
        map.insert("oauth_signature_method", "HMAC-SHA1".to_string());
        map.insert("oauth_timestamp", self.timestamp.to_string());
        map.insert("oauth_token", self.access_token.to_string());
        map.insert("oauth_version", "1.0".to_string());

        let mut parameter_string = String::new();
        for (key, value) in map.iter() {
            parameter_string.push_str(&format!("&{}={}", key, value));
        }

        let signature_base_string = match self.method {
            HttpMethod::Get => format!("{}&{}&{}", "GET", percent_encode(self.uri), percent_encode(&parameter_string[1..])),
            HttpMethod::Head => format!("{}&{}&{}", "HEAD", percent_encode(self.uri), percent_encode(&parameter_string[1..])),
            HttpMethod::Post => format!("{}&{}&{}", "POST", percent_encode(self.uri), percent_encode(&parameter_string[1..])),
        };
        let signing_key = format!("{}&{}", percent_encode(self.consumer_secret), percent_encode(self.access_token_secret));

        let mut hmac = Hmac::new(Sha1::new(), &signing_key.into_bytes());
        hmac.input(&signature_base_string.into_bytes());
        hmac.result().code().to_base64(base64::STANDARD)
    }
}

fn percent_encode<'a>(value: &'a str) -> String {
    value.chars().map(|x| {
        match x {
            '0'...'9' | 'A'...'Z' | 'a'...'z' | '-' | '.' | '_' | '~' => format!("{}", x),
            _ => format!("%{:X}", x as u8),
        }
    }).collect()
}

