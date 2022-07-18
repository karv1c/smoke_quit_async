use std::collections::HashMap;

use hyper::header::HeaderValue;

pub fn parse_cookie(s: Option<&HeaderValue>) -> Option<HashMap<&str, &str>> {
    let cookie = s?;
    let mut map = HashMap::new();
    let x = cookie.to_str().unwrap();
    let split = x.split(';');
    for s in split {
        match s.find('=') {
            Some(u) => {
                let k = &s[..u];
                let v = &s[u + 1..];
                map.insert(k, v);
            }
            None => println!("No key=value"),
        }
    }
    Some(map)
}
pub fn parse_cookie_s(s: Option<&HeaderValue>) -> Option<HashMap<String, String>> {
    let cookie = s?;
    let mut map = HashMap::new();
    let x = cookie.to_str().unwrap();
    let split = x.split(';');
    for s in split {
        match s.find('=') {
            Some(u) => {
                let k: String = s[..u].to_owned();
                let v = s[u + 1..].to_owned();
                map.insert(k, v);
            }
            None => println!("No key=value"),
        }
    }
    Some(map)
}
pub fn get_sessionid_from_cookie<'a>(s: Option<HashMap<&str, &'a str>>) -> Option<&'a str> {
    let session_id = s?.get("sessionid")?.to_owned();
    Some(session_id)
}
pub fn get_sessionid_from_cookie_s(s: Option<HashMap<String, String>>) -> Option<String> {
    let session_id = s?.get("sessionid")?.to_owned();
    Some(session_id)
}
