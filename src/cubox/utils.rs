use std::collections::HashMap;
use reqwest::header::HeaderMap;

pub fn construct_headers(token: String) -> HeaderMap {
  let mut map = HashMap::new();
  map.insert("authorization".to_string(), token.to_string());
  let headers: HeaderMap = (&map).try_into().expect("valid headers");
  headers
}