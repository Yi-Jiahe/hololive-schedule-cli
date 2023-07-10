# hololive-schedule-cli

A CLI tool for requesting stream schedules implemented in Rust

## Data source

Stream schedules are obtained from the [holodex-api](https://holodex.stoplight.io/).

holofans/holoapi support was removed because the service does not appear to be actively maintained.

## Installation

You can choose to compile the binary from source or download the precompiled binaries for tagged versions on the [releases](https://github.com/Yi-Jiahe/hololive-schedule-cli/releases) page.

## Config

First time setup intializes the initial config and will prompt for a Holodex API token. Instructions on how to obtain the token can be found [here](https://holodex.stoplight.io/docs/holodex/f4e6fa31af431-getting-started#obtaining-api-key).

Config files are stored at ~/.holo-schedule/.config in TOML format.

Example config:
```TOML
holodex_api_token = "REDACTED_TOKEN"

[format]
channel_name_col_length = 25
stream_title_name_col_length = 70
```

## Example usage

### Get streams live/going live in the next 12 hours

`holo-schedule`

Default config is to look back 12 hours and forwards 12 hours, showing only live and upcoming streams.

### Get video id of Ina's streams which are live/going live in the next 12 hours

`holo-schedule | grep "Ina" | awk 'BEGIN { FS = "\t" } ; { print $3 }'`

Columns are delimited by the `\t` character. The video id is the third column. Hopefully none of the channels use `\t` in their names.

Be sure to use a portion of the channel name that is included in the output. If the bit of the channel name used in the search is cut off, you can increase the width in the config.
