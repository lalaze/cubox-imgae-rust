use std::collections::HashMap;
use reqwest::header::HeaderMap;
use reqwest::header::{ REFERRER_POLICY };

pub fn construct_headers(token: String) -> HeaderMap {
  let mut map = HashMap::new();
  map.insert("authorization".to_string(), token.to_string());
  let headers: HeaderMap = (&map).try_into().expect("valid headers");
  headers
}

pub fn construct_headers2() -> HeaderMap {
  let mut map = HashMap::new();
  map.insert("user-agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36".to_string());
  let headers: HeaderMap = (&map).try_into().expect("valid headers");
  headers
}