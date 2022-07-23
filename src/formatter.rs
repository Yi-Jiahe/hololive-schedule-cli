extern crate unicode_width;

use unicode_width::UnicodeWidthStr;
use ansi_term::Colour::{Red, RGB};

const CHANNEL_NAME_WIDTH: usize = 30;

pub enum LiveStatus {
    Ended,
    Live,
    Upcoming,
}

pub fn format_line(start: String, channel_name: String, stream_title: String, live_status: LiveStatus) {

    let channel_name_width = UnicodeWidthStr::width(&channel_name[..]);

    let channel_name_col = if CHANNEL_NAME_WIDTH >= channel_name_width {
        let channel_name_padding = " ".repeat(CHANNEL_NAME_WIDTH - channel_name_width);
        format!("{}{}", channel_name, channel_name_padding)
    } else {
        /* TODO: 
        1) Split string between characters
        2) Format length
        println!("{:<30} {} {}", start, &channel_name[..CHANNEL_NAME_WIDTH], stream_title)
        */
        channel_name
    };

    let line = format!("{:<30} {} {}", start, channel_name_col, stream_title);

    // println!("{}", line);

    /*
    TODO:
    Binary compiled for Windows only shows color for Powershell. Git Bash shows the ansi characters.
    */
    match live_status {
        LiveStatus::Ended => println!("{}", RGB(105, 105, 105).paint(line)),
        LiveStatus::Live => println!("{}", Red.paint(line)),
        LiveStatus::Upcoming => println!("{}", line),
    };
}