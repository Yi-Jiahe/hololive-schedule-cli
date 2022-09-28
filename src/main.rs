use chrono::prelude::*;

const TOKEN: &str = env!("HOLODEX_API_TOKEN");

use holodex::model::{
    builders::VideoFilterBuilder, ExtraVideoInfo, Language, Organisation,
    VideoSortingCriteria, VideoType, VideoChannel, VideoLiveInfo, VideoStatus
};

use holo_schedule::formatter::{LiveStatus, format_line};

fn main() {
    let client = match holodex::Client::new(&TOKEN) {
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
                println!("{}", format_line(start.format("%e %b %T").to_string(), channel_min.name.clone(), stream.title.clone(), live_status));
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
