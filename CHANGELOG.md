# Unreleased
## Changes
- Update rust edition to 2024

# v0.8.1
- lock file update

# v0.8.0
## Changes
- update pass-it-on to v0.16.0
- switch to tracing from log for logging

# v0.7.0
## Breaking Changes
- bump pass-it-on to 0.15.0 which contains breaking changes

# v0.6.3
## Changes
- Added additional `LogLevel` values
- Update simple_logger to version 5
- Update pass-it-on to version 0.14.4

# v0.6.2
## Changes
- Add startup delay option to CLI

# v0.6.1
## Changes
- Add startup delay to Dockerfile control by `DELAY_STARTUP` environment variable

# v0.6.0
## Changes
- Add workflows
- Add Dockerfile
- Change crate name

# v0.5.0
## Breaking Changes
- bump pass-it-on to 0.14.2 which contains breaking changes

# v0.4.0
## Breaking Changes
- bump pass-it-on to 0.11.0 which is not backwards compatible

# v0.3.0
## Features
- Add ability to configure multiple notification name with different filters
- monitor and client now configurable in a single TOML file

## Changes
- change crate name to `pio-minecraft-server-monitor`
- update `pass-it-on` to 0.9.0 and make necessary changes
- Fix tests

# v0.2.3
## Changes
- Update lock file

# v0.2.2
## Changes
- Update lock file

# v0.2.1
## Changes
- Use defaults in `clap` CLI

# v0.2.0
## Features
- Use `pass-it-on` library for increased flexibility and ability to send to multiple endpoints 

# v0.1.0
## Features
- Basic functionality of monitoring log and send via Discord webhook.
