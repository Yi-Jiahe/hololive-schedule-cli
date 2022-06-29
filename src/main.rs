use holo_schedule::schedule_dot_hololive_scraper;
use holo_schedule::holoapi_wrapper;

fn main() {
    let args = Cli::parse();

    holoapi_wrapper::get_live(); 
}

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[clap(default_value = "", short = 'p', long = "pattern")]
    pattern: String,
}
