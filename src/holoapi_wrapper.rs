use serde::Deserialize;

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
struct LiveResponse {
    live: Vec<LiveVideo>,
    upcoming: Vec<LiveVideo>,
    ended: Vec<LiveVideo>,
    cached: bool,
}

pub fn get_live() {
    let response = reqwest::blocking::get(format!("{}{}?{}", HOST, "/live", "hide_channel_desc=1")).unwrap();
    let deserialized_response = response.json::<LiveResponse>();
    match deserialized_response {
        Result::Ok(LiveResponse { live, upcoming, ended, .. }) => {
            println!("--- Live ---");
            for video in live {
                if let Some(start) = video.live_start {
                    println!("{} {} {}", start, video.channel.name, video.title)
                }
            }
            println!("");

            println!("--- Upcoming ---");
            for video in upcoming {
                if let Some(start) = video.live_schedule {
                    println!("{} {} {}", start, video.channel.name, video.title)
                }
            }
            println!("");

            println!("--- Ended ---");
            for video in ended {
                if let Some(start) = video.live_start {
                    println!("{} {} {}", start, video.channel.name, video.title)
                }
            }
            println!("");
        }
        Result::Err(Error) => println!("Error")
    }

}