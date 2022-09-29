extern crate unicode_width;

use ansi_term::Colour::{Red, RGB};
use unicode_width::UnicodeWidthStr;

use crate::config::Config;

pub enum LiveStatus {
    Ended,
    Live,
    Upcoming,
    Other,
}

pub fn format_line(
    config: &Config,
    start: String,
    channel_name: String,
    stream_title: String,
    live_status: LiveStatus,
) -> String {
    // TODO: Trim channel and stream titles if too long

    // Format channel name to be the fixed length
    let channel_name_width = UnicodeWidthStr::width(&channel_name[..]);

    let channel_name_col = if config.format.channel_name_col_length > channel_name_width {
        let channel_name_padding =
            " ".repeat(config.format.channel_name_col_length - channel_name_width);
        format!("{}{}", channel_name, channel_name_padding)
    } else if config.format.channel_name_col_length < channel_name_width {
        trim_to_length(&channel_name[..], config.format.channel_name_col_length)
    } else {
        channel_name
    };

    let line = format!("{:<20} {} {}", start, channel_name_col, stream_title);

    /* Colour Line based on live status
     *  Red for live
     *  Black for upcoming
     *  Gray for ended (Depends on if gray is avaliable in the terminal) */
    match live_status {
        LiveStatus::Ended => RGB(200, 200, 200).paint(line).to_string(),
        LiveStatus::Live => Red.paint(line).to_string(),
        LiveStatus::Upcoming | LiveStatus::Other => line,
    }
}

fn trim_to_length(s: &str, desired_width: usize) -> String {
    let mut left = 0;
    let mut right = s.len();
    let mut pivot = right / 2;
    while !s.is_char_boundary(pivot) {
        pivot -= 1;
    }

    while right - 1 > left {
        let width = UnicodeWidthStr::width(&s[..pivot]);
        if width < desired_width - 2 {
            left = pivot + 1;
        } else if width > desired_width {
            right = pivot - 1;
        } else {
            break;
        }
        pivot = (right + left)/2;
        while !s.is_char_boundary(pivot) {
            pivot -= 1;
        }
    }

    let padding = " ".repeat(desired_width - UnicodeWidthStr::width(&s[..pivot]));

    format!("{}{}", &s[..pivot], padding)
}