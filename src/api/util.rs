use std::collections::HashMap;

use sha2::{Sha256, Digest};

pub fn encrypt(raw: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw);
    format!("{:x}", hasher.finalize())
}

pub  fn parse_cookies<'a>(raw: &'a str) -> HashMap<&'a str, &'a str> {
    raw.split(";").map(|cookie_str| {
        let cookie_parts = cookie_str.split("=").collect::<Vec<&'a str>>();
        if cookie_parts.len() != 2 {
            return None;
        }
        return Some((cookie_parts[0], cookie_parts[1]))
    }).filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect()
}