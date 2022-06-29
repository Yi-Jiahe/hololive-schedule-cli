use serde::Deserialize;

const HOST: &str = "https://api.holotools.app/v1";

#[derive(Deserialize, Debug)]
struct LiveVideo {
    id: i32,
    yt_video_key: Option<String>,
    bb_video_id: Option<String>,
    title: String,
    thumbnail: Option<String>,
    live_schedule: Option<String>,
    live_start: Option<String>,
    live_end: Option<String>,
    live_viewers: Option<i32>,
}

#[derive(Deserialize, Debug)]
struct LiveResponse {
    live: Vec<LiveVideo>,
    upcoming: Vec<LiveVideo>,
    ended: Vec<LiveVideo>,
    cached: bool,
}

pub fn get_live() {
    let response = dbg!(reqwest::blocking::get(format!("{}{}", HOST, "/live")).unwrap());
    let deserialized_response = dbg!(response.json::<LiveResponse>());
}