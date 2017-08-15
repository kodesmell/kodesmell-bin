extern crate reqwest;
extern crate serde_json;

use self::serde_json::{from_reader, Value, Error};
use std::io::Read;

pub fn create_project(name: &str) {
  let body = json!({
    "query": "mutation($input: CreateProjectInput) {
      createProject(input: $input) {
        id,
        name
      }
    }",
    "variables": {
      "input": {
        "name": name
      }
    }
  });

  let client = reqwest::Client::new().unwrap();
  let mut response = client.post("http://211.243.245.159:8000/graphql").unwrap()
    .json(&body).unwrap()
    .send().unwrap();

  let mut buf = String::new();
  response.read_to_string(&mut buf).expect("Failed to read response");

  let sss = buf.to_string();
  let v: Value = serde_json::from_str(&sss).unwrap();
  let id = &v["data"]["createProject"]["id"];
  println!("{:?}", id);

}
