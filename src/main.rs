#[macro_use] extern crate hyper;
use hyper::header::Headers;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use serde_json::Value;

use std::collections::HashMap;
use std::env;
use std::io::Read;

header! { (XAPIKey, "X-API-Key") => [String] }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Faction {
    faction_hash: u32,
    progress_to_next_level: u32,
    next_level_at: u32
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
    Ok(content.to_string())
}

fn make_headers() -> Headers {
    let api_key = env::var("BUNGIE_API_KEY").expect("missing bungie api env var");
    let mut headers = Headers::new();
    headers.set(XAPIKey(api_key.to_owned()));
    headers
}

fn get_factions(json_blob: &serde_json::Value) -> HashMap<String, Vec<Faction>> {
   let mut m = HashMap::new();
   for (key, val) in json_blob["Response"]["characterProgressions"]["data"].as_object().unwrap().iter() {
     let mut factions: Vec<Faction> = Vec::new();

     for obj in val["factions"].as_object().unwrap().values() {
         let faction: Faction = serde_json::from_value(obj.clone()).unwrap();
         factions.push(faction);
     }
     m.insert(key.to_string(), factions);
   }
   m
}


fn main() {
    let member_id = get_member_id(2, &"guubu").expect("Member ID not found");
    println!("{:?}", member_id);
    let profile = get_profile(2, &member_id).unwrap();
    let v: Value = serde_json::from_str(profile.as_str()).unwrap();
    let factions = get_factions(&v);
    for (character, fs) in factions {
        println!("{}: {:?}", character, fs);
    }
}
