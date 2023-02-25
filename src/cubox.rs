use serde::{Deserialize, Serialize};
use reqwest::multipart::{Form, Part};
use serde_json::Value;
use uuid::Uuid;
use std::fs::File;
use std::io::copy;
use std::path::Path;
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

pub async fn get_img(data: &Vec<Value>) -> () {
  let client = reqwest::Client::new();
  for x in data.iter() {
    let url = x.as_object().unwrap().get("targetURL").unwrap().as_str().unwrap();
    let filename = format!(r#"./file/{}"#, Uuid::new_v4().to_string());

    let res = client.get(url).send().await.unwrap();

    if res.status().is_success() {
      let mut dest_file = File::create(&Path::new(&filename)).unwrap();
      copy(&mut res.bytes().await.unwrap().as_ref(), &mut dest_file).unwrap();
    }

  }
}

#[tokio::main]
pub async fn get_box(name: String, p:String) -> ()   {
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
  let res2 = client.get("https://cubox.pro/c/api/v2/search_engine/my?asc=false&page=1&filters=&groupId=ff8080818630434a0186346168e779af&archiving=false")
  .headers(utils::construct_headers(token.to_string())).send().await.unwrap();
  let json_text = res2.text().await.unwrap();
  let json: Value = serde_json::from_str(&json_text).unwrap();

  let page_count =  json.get("pageCount").unwrap().as_u64().unwrap();

  let data = json.get("data").unwrap().as_array().unwrap();

  get_img(data).await;

  // if page_count > 1 {
  //   for i in 0..page_count {
  //     if (i != 0) {
  //       let url = format!(r#"https://cubox.pro/c/api/v2/search_engine/my?asc=false&page={}&filters=&groupId=ff8080818630434a0186346168e779af&archiving=false"#, i+1);
  //       let res = client.get(url).headers(utils::construct_headers(token.to_string())).send().await.unwrap();
  //       let json_text = res.text().await.unwrap();
  //       let json: Value = serde_json::from_str(&json_text).unwrap();
  //       let data2 = json.get("data").unwrap().as_array().unwrap();
  //     }
  //   }
  // }

  // // vec -> hashmap
  // for x in json_value.iter() {
  //     treeTypeMap.insert(x.gid, x.title.clone());
  // }

  // Ok(treeTypeMap)
  // Ok(())
}