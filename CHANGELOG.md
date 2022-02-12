# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]



## [0.0.3] - 2022-02-12

### Added

- `read_raw` and `read accel` added
- `AccelConfig` added

### Changed
- enum names are now CamelCase
- MSA301 struct holds the AccelConfig struct
- `begin_accel` initializes the sensor with configuration supplied in `new`
- raw reading function returns an array

### Fixed
- wrong readings caused by unsigned vs unsigned

## [0.0.2] - 2022-02-08

### Added
- `get_device_id` function to read the PART_ID
- RaspberryPi example using `get_device_id`
- `new_data_int` interrupt enabling/disabling
- `freefall_int` interrupt enabling/disabling
- `single_tap_int` interrupt enabling/disabling
- `double_tap_int` interrupt enabling/disabling
- `orient_tap_int` interrupt enabling/disabling
- `active_xaxis_int` interrupt enabling/disabling
- `active_yaxis_int` interrupt enabling/disabling
- `active_zaxis_int` interrupt enabling/disabling
- `set_datarate`, `set_bandwidth` and `set_power_mode` added to config.rs

### Changed
- Use simple I2C-only interface based on the PCF8563 crate instead of the more complex solution with SPI, multiple addresses etc.

## [0.0.1] - 2022-02-03

### Added
- basic crate structure and necessary modules

[0.0.1]: https://github.com/nebelgrau77/msa301-rs/releases/tag/v0.0.1
