# hololive-schedule-cli

A CLI tool for requesting stream schedules implemented in Rust

## Data source

Stream schedules are obtained from the [holodex-api](https://holodex.stoplight.io/).

holofans/holoapi support was removed because the service does not appear to be actively maintained.

## Installation

You can choose to compile the binary from source or download the precompiled binaries for tagged versions on the [releases](https://github.com/Yi-Jiahe/hololive-schedule-cli/releases) page.

### Config

First time setup intializes the initial config and will prompt for a Holodex API token. Instructions on how to obtain the token can be found [here](https://holodex.stoplight.io/docs/holodex/f4e6fa31af431-getting-started#obtaining-api-key).

Config files are stored at ~/.holo-schedule/.config in TOML format.



