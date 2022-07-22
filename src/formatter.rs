extern crate unicode_width;

use unicode_width::UnicodeWidthStr;

const CHANNEL_NAME_WIDTH: usize = 30;

pub fn format_line(start: String, channel_name: String, stream_title: String) {

    let channel_name_width = UnicodeWidthStr::width(&channel_name[..]);

    if CHANNEL_NAME_WIDTH >= channel_name_width {
        let channel_name_padding = " ".repeat(CHANNEL_NAME_WIDTH - channel_name_width);
        println!("{:<30} {}{} {}", start, channel_name, channel_name_padding, stream_title)
    } else {
        /* TODO: 
        1) Split string between characters
        2) Format length
        println!("{:<30} {} {}", start, &channel_name[..CHANNEL_NAME_WIDTH], stream_title)
        */
        println!("{:<30} {} {}", start, channel_name, stream_title)
    }


}