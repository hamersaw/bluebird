extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate time;

pub mod stream;
pub mod oauth;

pub fn percent_encode(string: String) -> String {
    string.chars().map(|x| {
        match x {
            '0'...'9' | 'A'...'Z' | 'a'...'z' | '-' | '.' | '_' | '~' => format!("{}", x),
            _ => format!("%{:X}", x as u8),
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("Ladies + Gentleman".to_string()), "Ladies%20%2B%20Gentleman".to_string());
        assert_eq!(percent_encode("Dogs, Cats, & Mice!".to_string()), "Dogs%2C%20Cats%2C%20%26%20Mice%21".to_string());
    }
}
