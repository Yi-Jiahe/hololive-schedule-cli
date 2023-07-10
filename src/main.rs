use std::fs;
use std::io;

extern crate dirs;
extern crate toml;

use chrono::prelude::*;
use chrono::Duration;

use holodex::model::{
    builders::VideoFilterBuilder, ExtraVideoInfo, Language, Organisation, VideoChannel,
    VideoLiveInfo, VideoSortingCriteria, VideoStatus, VideoType,
};

use holo_schedule::config::Config;
use holo_schedule::formatter::{format_line, LiveStatus};

fn main() {
    let args = Arguments::parse();

    let home_dir = dirs::home_dir().unwrap();
    let config_dir = format!("{}/.holo-schedule", home_dir.display());

    match fs::create_dir(&config_dir) {
        Ok(()) => {}
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            _ => {
                panic!("{}", err)
            }
        },
    };

    let config_file_path = format!("{}/.config", config_dir);

    // Get config from file or create new config & save to file
    let mut config = match Config::from_file(&config_file_path) {
        Ok(config) => config,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                let config = Config::new();
                config.write_to_file(&config_file_path).unwrap();
                config
            }
            _ => panic!("{}", err),
        },
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

        config.write_to_file(&config_file_path).unwrap();
    }

    let client = match holodex::Client::new(&config.holodex_api_token) {
        Result::Ok(client) => client,
        Result::Err(err) => {
            match err {
                holodex::errors::Error::InvalidApiToken => {
                    panic!("The API token provided to the client is invalid.")
                }
                holodex::errors::Error::HttpClientCreationError(err) => {
                    panic!("An error occurred while creating the HTTP client.\n{}", err)
                }
                // Client::new() only returns the above 2 types of error
                _ => panic!("{}", err),
            }
        }
    };

    let from = chrono::Utc::now()
        .checked_sub_signed(Duration::hours(args.previous_hours as i64))
        .unwrap();

    let mut statuses = Vec::new();
    if args.ended { statuses.push(VideoStatus::Past) };
    if args.live { statuses.push(VideoStatus::Live) };
    if args.upcoming { statuses.push(VideoStatus::Upcoming) };
    
    let filter = VideoFilterBuilder::new()
        .organisation(Organisation::Hololive)
        .language(&[Language::All])
        .video_type(VideoType::Stream)
        .after(from)
        .max_upcoming_hours(args.max_upcoming_hours as u32)
        .status(&statuses)
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

    for video in results.iter().rev() {
        let start: DateTime<Local> = DateTime::from(match video.live_info {
            VideoLiveInfo {
                start_scheduled: _,
                start_actual: Some(start_actual),
                ..
            } => start_actual,
            VideoLiveInfo {
                start_scheduled: Some(start_scheduled),
                start_actual: None,
                ..
            } => start_scheduled,
            _ => video.available_at,
        });

        let live_status = match video.status {
            VideoStatus::Upcoming => LiveStatus::Upcoming,
            VideoStatus::Live => LiveStatus::Live,
            VideoStatus::Past => LiveStatus::Ended,
            _ => LiveStatus::Other,
        };

        match &video.channel {
            VideoChannel::Min(channel_min) => {
                println!(
                    "{}",
                    format_line(
                        &config,
                        start.format("%e %b %H:%M").to_string(),
                        channel_min.name.clone(),
                        video.id.to_string(),
                        video.title.clone(),
                        live_status
                    )
                );
            }
            _ => (),
        }
    }
}

use clap::{Parser};

/// A Command Line Application to retrieve a list of streams in a given time period
#[derive(Parser)]
struct Arguments {
    // How far in the past to display streams from
    #[clap(default_value = "12", short = 'p', long = "previous_hours")]
    previous_hours: f32,
    // How far in the future to display streams from
    #[clap(default_value = "12", short = 'u', long = "max_upcoming_hours")]
    max_upcoming_hours: f32,

    // Show ended streams
    #[clap(action, default_value_t = false, long = "ended")]
    ended: bool,
    // Show live streams (Default: True)
    #[clap(action, default_value_t = true, long = "live")]
    live: bool,
    // Show upcoming streams (Default: True)
    #[clap(action, default_value_t = true, long="upcoming")]
    upcoming: bool,
}
