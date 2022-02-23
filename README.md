# Rust MSA301 digital accelerometer driver

![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

A platform agnostic Rust driver for the MEMSensing Microsystems MSA301 digital accelerometer,
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

Inspired by and partially based on [another STMicroelectronics MEMS driver](https://github.com/lonesometraveler/lsm9ds1).  
[Adafruit's description and C++ driver](https://learn.adafruit.com/msa301-triple-axis-accelerometer) used as a reference.

This driver allows you to:
- read the device ID (0x13)
- create a new instance of the driver with a default or user-specified configuration
- configure datarate, bandwidth, power mode, range (full scale) and bit resolution, and enable/disable axes
- read the measurements 
- enable/disable interrupts and configure latching
- read the interrupts' status
- configure INT pin 

## WORK IN PROGRESS:

__This library is work in progress. Not all features are implemented yet. Some functions are for test purposes only and will be removed later. Only the I2C interface is implemented. Contributions are welcome!__

### TO DO:
 
- [ ] add interrupts mapping to INT pin 
- [ ] add active detection settings (threshold, duration)
- [ ] add tap detection settings
- [ ] add orientation recognition settings
- [ ] add freefall detection settings
- [ ] getter functions for various configuration elements
- [ ] add nore examples (STM32/RP2040/nRF52/ATSAMD, RasPi)
- [ ] better documentation

## The device

The MSA301 is a small and low cost triple-axis accelerometer. It features:
* three axis sensing with 14-bit resolution
* ±2g/±4g/±8g/±16g selectable scaling
* I2C interface on fixed I2C address
* Interrupt output
* Multiple data rate options from 1 Hz to 500 Hz
* As low as 2uA current draw in low power mode (just the chip itself, not including any supporting circuitry)
* Tap, Double-tap, orientation & freefall detection

Datasheet: [MSA301](hhttps://github.com/adafruit/Adafruit_MSA301/blob/master/MSA301-V1.0-ENG.PDF)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [examples]

[examples]: https://github.com/nebelgrau77/lps22hb-rs/tree/main/examples

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/nebelgrau77/lps22hb-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
