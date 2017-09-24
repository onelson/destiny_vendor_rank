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
    let client = reqwest::Client::new().map_err(|e| e.to_string())?;
    let resp = client.get(url.as_str()).map_err(|e| e.to_string())?.headers(make_headers()).send();
    let mut content = String::new();
    let _ = resp.unwrap().read_to_string(&mut content);
    let v: Value = serde_json::from_str(content.as_str()).map_err(|e| e.to_string()).unwrap();
    let mem_id = v["Response"][0]["membershipId"].as_str().unwrap().to_string();
    Ok(mem_id)
}

fn get_profile(platform: i32, user: &str) -> String {
    let url = format!("{}{}/Profile/{}", BASE, platform, &"id");

    url
}

fn make_headers() -> Headers {
    let api_key = env::var("BUNGIE_API_KEY").expect("missing bungie api env var");
    let mut headers = Headers::new();
    headers.set(XAPIKey(api_key.to_owned()));
    headers
}

fn main() {
    let resp = get_member_id(2, &"guubu");
    println!("{:?}", resp);
    println!("{}", get_profile(2, &"guubu"));
}
