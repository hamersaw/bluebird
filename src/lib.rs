extern crate crypto;
extern crate hyper;
extern crate rand;
extern crate rustc_serialize;
extern crate time;

pub mod client;

#[doc(hidden)]
pub mod oauth;
pub mod request;

pub use client::Client;
pub use request::BluebirdRequest;

/// Returns the percent encoded representation of the string parameter.
///
/// # Examples
/// ```
/// let percent_encoded_str = percent_encode("This is a string to percent encode".to_string());
/// ```
pub fn percent_encode(string: String) -> String {
    string.chars().map(|x| {
        match x {
            '0'...'9' | 'A'...'Z' | 'a'...'z' | '-' | '.' | '_' | '~' => format!("{}", x),
            _ => format!("%{:X}", x as u8),
        }
    }).collect()
}

