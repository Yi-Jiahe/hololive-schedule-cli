fn main() {
    let args = Cli::parse();

    let response = reqwest::blocking::get(
        "https://schedule.hololive.tv/simple",
    )
    .unwrap()
    .text()
    .unwrap();
    dbg!(response);
}

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[clap(default_value="", short = 'p', long = "pattern")]
    pattern: String,
}

