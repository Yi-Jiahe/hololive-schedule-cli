fn main() {
    let args = Cli::parse();

    let response = reqwest::blocking::get("https://schedule.hololive.tv/simple").unwrap();

    let body = response.text().unwrap();

    let document = scraper::Html::parse_document(&body);

    let content_selector = scraper::Selector::parse(".tab-content").unwrap();
    let link_selector = scraper::Selector::parse("a").unwrap();

    let content = document.select(&content_selector).next().unwrap();
    dbg!(content.value());

    for element in content.select(&link_selector) {
        if let Some(youtube_link) = element.value().attr("href") {
            let parsed_string = element.text().collect::<Vec<_>>()[0]
                .split(&['\n', ' '][..])
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            let icon = parsed_string.get(2);
            println!("{} {:20} {}", parsed_string[0], parsed_string[1], youtube_link);
        }
    }
}

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[clap(default_value = "", short = 'p', long = "pattern")]
    pattern: String,
}
