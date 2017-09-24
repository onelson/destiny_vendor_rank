#[macro_use] extern crate hyper;
use hyper::header::Headers;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use serde_json::Value;
use serde_json::error;

use std::collections::HashMap;
use std::env;
use std::io::Read;

header! { (XAPIKey, "X-API-Key") => [String] }

#[derive(Deserialize)]
struct MemResponse {
    response: Vec<HashMap<String, String>>
}


const BASE: &'static str = "https://bungie.net/Platform/Destiny2/";

fn get_member_id(platform: i32, user: &str) -> Result<String, String> {
    let url = format!("{}SearchDestinyPlayer/{}/{}/", BASE, platform, user);
    let resp = make_request(&url);
    let mut content = String::new();
    let _ = resp.unwrap().read_to_string(&mut content);
    let v: Value = serde_json::from_str(content.as_str()).map_err(|e| e.to_string()).unwrap();
    let mem_id = v["Response"][0]["membershipId"].as_str().unwrap().to_string();
    Ok(mem_id)
}

fn make_request(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new()?;
    let headers = make_headers();
    println!("{:?}", headers);
    client.get(url)?.headers(headers).send()
}

fn get_profile(platform: i32, member_id: &str) -> Result<String, String> {
    let url = format!("{}{}/Profile/{}/?components=200,202", BASE, platform, member_id);
    println!("{:?}", url);
    let resp = make_request(&url);
    let mut content = String::new();
    let _ = resp.unwrap().read_to_string(&mut content);
    println!("{:?}", content);
    Ok(content.to_string())
}

fn make_headers() -> Headers {
    let api_key = env::var("BUNGIE_API_KEY").expect("missing bungie api env var");
    let mut headers = Headers::new();
    headers.set(XAPIKey(api_key.to_owned()));
    headers
}

fn main() {
    let member_id = get_member_id(2, &"guubu").expect("Member ID not found");
    println!("{:?}", member_id);
    println!("{:?}", get_profile(2, &member_id));
}
