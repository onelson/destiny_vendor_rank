const BASE: &'static str = "https://bungie.net/Platform/Destiny2/";

fn get_member_id(platform: i32, user: &str) -> String {
    let url = format!("{}SearchDestinyPlayer/{}/{}/", BASE, platform, user);
    url
}

fn get_profile(platform: i32, user: &str) -> String {
    let url = format!("{}{}/Profile/{}", BASE, platform, &"id");
    url
}

fn main() {
    println!("{}", get_member_id(2, &"guubu"));
    println!("{}", get_profile(2, &"guubu"));
}
