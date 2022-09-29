use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

extern crate dirs;
extern crate toml;

use chrono::prelude::*;

use holodex::model::{
    builders::VideoFilterBuilder, ExtraVideoInfo, Language, Organisation,
    VideoSortingCriteria, VideoType, VideoChannel, VideoLiveInfo, VideoStatus
};

use holo_schedule::config::Config;
use holo_schedule::formatter::{LiveStatus, format_line};

fn main() {
    let home_dir = dirs::home_dir().unwrap();
    let config_dir = format!("{}/.holo-schedule", home_dir.display());

    match fs::create_dir(&config_dir) {
        Ok(()) => {},
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {},
            _ => {panic!("{}", err)}
        }
    };

    let mut config = Config::new();
    config = dbg!(config);

    let mut config_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(format!("{}/.config", &config_dir)).unwrap();

    let mut contents = String::new();
    config_file.read_to_string(&mut contents).unwrap();

    config = toml::from_str(&contents).unwrap();

    if config.holodex_api_token == "" {
        // TODO: Prompt for token
        panic!("Please add api token")
    }

    let client = match holodex::Client::new(&config.holodex_api_token) {
        Result::Ok(client) => client,
        Result::Err(err) => {
            match err {
                holodex::errors::Error::InvalidApiToken => {
                    panic!("The API token provided to the client is invalid.")
                }
                holodex::errors::Error::HttpClientCreationError(err) => panic!(
                    "An error occurred while creating the HTTP client.\n{}",
                    err
                ),
                // Client::new() only returns the above 2 types of error
                _ => panic!("{}", err),
            }
        }
    };

    let args = Cli::parse();

    let filter = VideoFilterBuilder::new()
    .organisation(Organisation::Hololive)
    .language(&[Language::All])
    .video_type(VideoType::Stream)
    .max_upcoming_hours(24)
    .include(&[ExtraVideoInfo::Description, ExtraVideoInfo::LiveInfo])
    .sort_by(VideoSortingCriteria::StartScheduled)
    .build();

    let results = match client.videos(&filter) {
        Result::Ok(results) => results,
        Result::Err(err) => {
            match err {
                holodex::errors::Error::ApiRequestFailed { source, endpoint } => panic!(
                    "An error occurred while sending a request to the API.\n{} {}",
                    source, endpoint
                ),
                holodex::errors::Error::InvalidResponse { source, endpoint } => panic!(
                    "The API returned a faulty response or server error.\n{} {}",
                    source, endpoint
                ),
                // Client::new() only returns the above 2 types of error
                _ => panic!("{}", err),
            }
        }
    };

    for stream in results.iter().rev() {
        let start: DateTime<Local> = DateTime::from(match stream.live_info {
            VideoLiveInfo{ start_scheduled: _, start_actual: Some(start_actual), .. } => start_actual,
            VideoLiveInfo{ start_scheduled: Some(start_scheduled), start_actual: None, .. } => start_scheduled,
            _ => panic!("Could not get start time"),
        });

        let live_status = match stream.status {
            VideoStatus::Upcoming => LiveStatus::Upcoming,
            VideoStatus::Live => LiveStatus::Live,
            VideoStatus::Past => LiveStatus::Ended, 
            _ => LiveStatus::Other,
        };

        match &stream.channel {
            VideoChannel::Min(channel_min) => {
                let title = format!("{:<10} {}", stream.id.to_string(), stream.title.clone());
                println!("{}", format_line(start.format("%e %b %T").to_string(), channel_min.name.clone(), title, live_status));
            },
            _ => (),
        }
    }
}

use clap::Parser;

/// List streams.
#[derive(Parser)]
struct Cli {
    #[clap(default_value = "24", long = "max_upcoming_hours")]
    max_upcoming_hours: f32,
    #[clap(default_value = "11", long = "lookback_hours")]
    lookback_hours: f32,
}
