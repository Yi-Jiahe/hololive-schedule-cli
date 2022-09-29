use std::fs;
use std::io;

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
    let args = Arguments::parse();

    let home_dir = dirs::home_dir().unwrap();
    let config_dir = format!("{}/.holo-schedule", home_dir.display());

    match fs::create_dir(&config_dir) {
        Ok(()) => {},
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {},
            _ => {panic!("{}", err)}
        }
    };

    let config_file_path = format!("{}/.config", config_dir);

    // Get config from file or create new config & save to file
    let mut config = match Config::from_file(&config_file_path) {
        Ok(config) => config,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                let config = Config::new();
                config.write_to_file(&config_file_path);
                config
            },
            _ => panic!("{}", err)
        }
    };

    // Get API token from user if absent
    if config.holodex_api_token == "" {
        println!("Please provide Holodex API token:");

        let mut token = String::new();

        io::stdin()
            .read_line(&mut token)
            .expect("Failed to read line");
        token = token.trim().to_string();

        config.holodex_api_token = token;

        config.write_to_file(&config_file_path);
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

    let filter = VideoFilterBuilder::new()
    .organisation(Organisation::Hololive)
    .language(&[Language::All])
    .video_type(VideoType::Stream)
    .max_upcoming_hours(args.max_upcoming_hours as u32)
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
                println!("{}", format_line(&config, start.format("%e %b %T").to_string(), channel_min.name.clone(), title, live_status));
            },
            _ => (),
        }
    }
}

use clap::Parser;

/// List streams.
#[derive(Parser)]
struct Arguments {
    #[clap(default_value = "24", short='u', long = "max_upcoming_hours")]
    max_upcoming_hours: f32,
    #[clap(default_value = "11", long = "lookback_hours")]
    lookback_hours: f32,
}
