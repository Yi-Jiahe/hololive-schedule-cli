extern crate dotenv;

use dotenv::dotenv;

use holodex::model::{
    builders::VideoFilterBuilder, ExtraVideoInfo, Language, Organisation,
    VideoSortingCriteria, VideoType
};

use holo_schedule::holoapi_wrapper;

fn main() {
    dotenv().ok();

    let token = std::env::var("HOLODEX_API_TOKEN").unwrap();

    let client = match holodex::Client::new(&token) {
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
    .organisation(Organisation::Independents)
    .language(&[Language::Japanese])
    .video_type(VideoType::Stream)
    .max_upcoming_hours(24)
    .include(&[ExtraVideoInfo::Description])
    .sort_by(VideoSortingCriteria::StartScheduled)
    .limit(5)
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

    for stream in results {
        println!("{}", stream.title);
    }

    let args = Cli::parse();

    holoapi_wrapper::get_live(args.max_upcoming_hours, args.lookback_hours);
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
