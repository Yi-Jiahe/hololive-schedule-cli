use serde::Deserialize;

use crate::formatter::format_line;

const HOST: &str = "https://api.holotools.app/v1";

#[derive(Deserialize, Debug)]
struct Channel {
    id: i32,
    yt_channel_id: Option<String>,
    bb_space_id: Option<String>,
    name: String,
    description: Option<String>,
    photo: Option<String>,
    published_at: String,
    twitter_link: Option<String>,
}

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
    channel: Channel,
}

#[derive(Deserialize, Debug)]
struct Success {
    live: Vec<LiveVideo>,
    upcoming: Vec<LiveVideo>,
    ended: Vec<LiveVideo>,
    cached: bool,
}

#[derive(Deserialize, Debug)]
struct Error {
    message: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum LiveResponse {
    Success(Success),
    Error(Error)
}

pub fn get_live(max_upcoming_hours: f32, lookback_hours:f32) {
    let query_string = format!("hide_channel_desc=1&max_upcoming_hours={}&lookback_hours={}", max_upcoming_hours, lookback_hours);
    let response = reqwest::blocking::get(format!("{}{}?{}", HOST, "/live", query_string)).unwrap();
    let deserialized_response = response.json::<LiveResponse>();
    match deserialized_response {
        Result::Ok(live_response) => {
            match live_response{
                LiveResponse::Success(Success {mut live, mut upcoming, mut ended, ..}) => {
                    println!("--- Ended ---");
                    ended.sort_by(|a, b| a.live_start.cmp(&b.live_start));
                    for video in ended {
                        if let Some(start) = video.live_start {
                            format_line(start, video.channel.name, video.title);
                        }
                    }
                    println!("");

                    println!("--- Live ---");
                    live.sort_by(|a, b| a.live_start.cmp(&b.live_start));
                    for video in live {
                        if let Some(start) = video.live_start {
                            format_line(start, video.channel.name, video.title);
                        }
                    }
                    println!("");
        
                    println!("--- Upcoming ---");
                    upcoming.sort_by(|a, b| a.live_schedule.cmp(&b.live_schedule));
                    for video in upcoming {
                        if let Some(start) = video.live_schedule {
                            format_line(start, video.channel.name, video.title);
                        }
                    }
                    println!("");
                }
                LiveResponse::Error(error) => {
                    println!("{}", error.message);

                }
            }
        }
        Result::Err(error) => println!("Error: {}", error)
    }
}