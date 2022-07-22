use holo_schedule::holoapi_wrapper;

fn main() {
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
