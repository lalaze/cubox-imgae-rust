use std::os::unix::net::UnixDatagram;

use serde::{Deserialize, Serialize};
use reqwest::multipart::{Form, Part};
use serde_json::Value;
mod utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRsp {
  message: String,
  code: u64,
  data: Option<u64>,
  pub token: String,
  user_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Box {
  // "type": u64,
  user_search_engine_id: u64,
  title: String,
  description: Option<String>,
  target_url: String,
  resource_url: String,
  home_url: String,
  archive_name: String,
  content: Option<String>,
  article_name: String,
  article_word_count: u64,
  byline: String,
  cover: String,
  article_url: String,
  little_icon: String,
  archiving: bool,
  star_target: bool,
  has_mark: bool,
  is_read: Option<u64>,
  mark_count: u64,
  // tags: Vec<T>,
  // all_tags: Vec<T>,
  // marks: Vec<T>,
  group_id: String,
  group_name: String,
  create_time: String,
  update_time: String,
  status: String,
  finished: bool,
  in_black_or_white_list: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxRsp {
  message: String,
  code: u64,
  data: Vec<Box>
}

#[tokio::main]
pub async fn get_box(name: String, p:String) -> Result<(), Box<dyn std::error::Error>>   {
  let client = reqwest::Client::new();

  let form = Form::new()
  .part("userName", Part::text(name.to_owned()))
  .part("password", Part::text(p.to_owned()));
  
  let res =
    client.post("https://cubox.pro/c/api/login")
    .multipart(form).send().await.unwrap();
  
  let json: Value = res.json().await.unwrap();

  let token = json["token"].as_str().unwrap();

  // 先写死路径了
  let res2 = client.post("https://cubox.pro/c/api/v2/search_engine/my?asc=false&page=1&filters=&groupId=ff8080818630434a0186346168e779af&archiving=false")
  .headers(utils::construct_headers(token.to_string())).send().await.unwrap();

  let json2: Value = res.json().await.unwrap();
  let data= json["token"].as_str().unwrap();


  // // vec -> hashmap
  // for x in json_value.iter() {
  //     treeTypeMap.insert(x.gid, x.title.clone());
  // }

  // Ok(treeTypeMap)
  Ok(())
}