use reqwest::multipart::{Form, Part};
use reqwest::{ ClientBuilder };
use serde_json::Value;
use std::fs::File;
use std::io::copy;
use std::path::Path;
mod utils;

fn check_file_exixt(url: &String) -> bool {
  let path = Path::new(&url);
  path.exists()
} 

pub async fn get_img(data: &Vec<Value>, f: &String) -> () {
  let client = ClientBuilder::new().referer(false).build().unwrap();
  for x in data.iter() {
    let url = x.as_object().unwrap().get("targetURL").unwrap().as_str().unwrap();
    let parts: Vec<&str> = url.split('/').collect();
    let file_last = parts.last().unwrap();
    let mut filename: String;
    // 处理带sig的路径逻辑
    let file_last = if file_last.contains("?sig") {
      let aa: Vec<&str> = file_last.split('=').collect();
      filename = format!(r#"{}/{}.jpeg"#, f, aa.last().unwrap());
    } else {
      filename = format!(r#"{}/{}"#, f, file_last);
    };
    if !filename.contains("jpeg") & !filename.contains("jpg") {
      filename = format!(r#"{}.jpeg"#, filename)
    }
    if !check_file_exixt(&filename) {
      let res = client.get(url).headers(utils::construct_headers2()).send().await.unwrap();
      let mut dest_file: File;

      if res.status().is_success() {
        dest_file = File::create(&Path::new(&filename)).unwrap();
        copy(&mut res.bytes().await.unwrap().as_ref(), &mut dest_file).unwrap();
      } else {
        print!("{:?}", "error");
        print!("{:?}", url);
        print!("{}", "\n");
      }
    } else {
      print!("{:?}", "重复");
      print!("{:?}", url);
      print!("{}", "\n");
    }
  }
}

#[tokio::main]
pub async fn get_box(name: String, p:String, f: String) -> ()   {
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

  print!("{:?}", data.len());
  print!("{}", "\n");

  get_img(data, &f).await;

  if page_count > 1 {
    let mut data2: &Vec<Value>;
    let mut last_id = data.last().unwrap().as_object().unwrap().get("userSearchEngineID").unwrap();
    let mut json: Value;
    for i in 0..page_count {
      if (i != 0) {
        let url =
          format!(r#"https://cubox.pro/c/api/v2/search_engine/my?asc=false&page={}&filters=&groupId=ff8080818630434a0186346168e779af&archiving=false&lastBookmarkId={}"#, i + 1, last_id.as_str().unwrap());
        let res = client.get(url).headers(utils::construct_headers(token.to_string())).send().await.unwrap();
        let json_text = res.text().await.unwrap();
        json = serde_json::from_str(&json_text).unwrap();
        data2 = json.get("data").unwrap().as_array().unwrap();
        print!("{:?}", data2.len());
        print!("{}", "\n");
        last_id = data2.last().unwrap().as_object().unwrap().get("userSearchEngineID").unwrap();
        get_img(data2, &f).await;
      }
    }
  }
}

#[tokio::main]
pub async fn testtest() {
  let client = ClientBuilder::new().referer(false).build().unwrap();
  let res = client.get("https://drscdn.500px.org/photo/1062414116/q%3D80_h%3D600/v2?sig=b3ef386d60b77cef6f32a004b9dff27763d9cc0dd626911eff07302c5ebdb05c").headers(utils::construct_headers2()).send().await.unwrap();
  print!("{:?}", res);
}