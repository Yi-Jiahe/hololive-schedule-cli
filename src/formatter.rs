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
    let desired_channel_name_col_width = config.format.channel_name_col_length;
    let channel_name_col = if desired_channel_name_col_width > channel_name_width {
        let padding = " ".repeat(desired_channel_name_col_width - channel_name_width);
        format!("{}{}", channel_name, padding)
    } else if desired_channel_name_col_width < channel_name_width {
        trim_to_length(&channel_name[..], desired_channel_name_col_width)
    } else {
        channel_name
    };

    let stream_title_width = UnicodeWidthStr::width(&stream_title[..]);
    let desired_stream_title_col_width = config.format.stream_title_name_col_length;
    let stream_title_col = if desired_stream_title_col_width > stream_title_width {
        let padding = " ".repeat(desired_stream_title_col_width - stream_title_width);
        format!("{}{}", stream_title, padding)
    } else if desired_stream_title_col_width < stream_title_width {
        trim_to_length(&stream_title[..], desired_stream_title_col_width)
    } else {
        stream_title
    };

    let line = format!("{:<15} {} {}", start, channel_name_col, stream_title_col);

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

// TODO: add a test for this function
// I think there is a case where it loops infinitely
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
        pivot = (right + left) / 2;
        while !s.is_char_boundary(pivot) {
            pivot -= 1;
        }
    }

    let padding = " ".repeat(desired_width - UnicodeWidthStr::width(&s[..pivot]));

    format!("{}{}", &s[..pivot], padding)
}
