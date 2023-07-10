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
    video_id: String,
    stream_title: String,
    live_status: LiveStatus,
) -> String {
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

    let line = format!("{:<15}\t{}\t{}\t{}", start, channel_name_col, video_id, stream_title_col);

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
    // Early return if s is equal to or shorter than the desired width
    let initial_width = UnicodeWidthStr::width(s);
    if initial_width == desired_width {
        return s.to_string();
    } else if initial_width < desired_width {
        return format!("{}{}", s, " ".repeat(desired_width-initial_width));
    }

    // Binary search to find character boundary to produce the width closest to the desired width
    let mut left = 0;
    let mut right = s.len();
    let mut pivot = right / 2;
    // Ensure that pivot is placed at character boundary to provide valid width
    while !s.is_char_boundary(pivot) {
        pivot -= 1;
    }

    while right > left {
        let width = UnicodeWidthStr::width(&s[..pivot]);
        if width < desired_width {
            left = pivot + 1;
            while !s.is_char_boundary(left) {
                left += 1;
            }
        } else if width > desired_width {
            right = pivot - 1;
            while !s.is_char_boundary(right) {
                right -= 1;
            }
        } else {
            return s[..pivot].to_string();
        }
        pivot = (right + left) / 2;
        // Move pivot to the character boundary on the left
        while !s.is_char_boundary(pivot) {
            pivot -= 1;
        }
    }

    // As it is possible that the desired width cannot be met
    // The boundary could produce a length slightly more or less than the desired width
    // If it is more, remove the extra character
    let width = UnicodeWidthStr::width(&s[..pivot]);
    if width > desired_width {
        pivot -= 1;
        while !s.is_char_boundary(pivot) {
            pivot -= 1;
        }
    }

    // Add padding to ensure that the resultant string is the desired width
    let padding = " ".repeat(desired_width - UnicodeWidthStr::width(&s[..pivot]));

    format!("{}{}", &s[..pivot], padding)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn standard_width_less_than() {
        assert_eq!("1234567   ", trim_to_length("1234567", 10));
    }
    
    #[test]
    fn standard_width_more_than() {
        assert_eq!("1234567890", trim_to_length("123456789012345", 10));
    }
    
    #[test]
    fn japanese_characters_more_than() {
        assert_eq!("【カメラ】", trim_to_length("【カメラ】ホロサマビンゴ7つクリア", 10));
    }

    #[test]
    fn japanese_characters_more_than_odd() {
        assert_eq!("【カoラ】 ", trim_to_length("【カoラ】ホロサマビンゴ7つクリア", 10));
    }
 
    #[test]
    fn japanese_characters_more_than_odd_pivot_right() {
        assert_eq!("【カメo】 ", trim_to_length("【カメo】ホロサマビンゴ7つクリア", 10));
    }
 
    #[test]
    fn japanese_characters_more_than_odd_pivot_left() {
        assert_eq!("【oメラ】 ", trim_to_length("【oメラ】ホロサマビンゴ7つクリア", 10));
    }

    // TODO: Add a test case which is able to trigger the block shifting the final cursor to the left
}  