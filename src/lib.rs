//! A platform agnostic driver to interface with MSA301 digital accelerometer module.
//!
//! This driver allows you to:

#![no_std]
//#![deny(warnings, missing_docs)]

pub mod sensor;
pub mod config;
pub mod fifo;
pub mod interrupt;
pub mod register;
use register::{Bitmasks, Registers};

//pub mod interface;
// use interface::Interface;

use embedded_hal as hal;
use hal::blocking::i2c::{Write, WriteRead};

/// Sensor's ID
// const PARTID: u8 = 0x13; // decimal value 19


/// I2C device address
const DEV_ADDR: u8 = 0b000100110;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2C(E),
    /// Invalid input data
    InvalidInputData,
}

/// Holds the driver instance with the selected interface
#[derive(Debug, Default)]
pub struct MSA301<I2C> {
    /// The concrete I2C device implementation
    i2c: I2C,
}

impl <I2C, E> MSA301<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance of the LPS25HB driver.
    pub fn new(i2c: I2C) -> Self {
        MSA301 { i2c }
    }

    /// Destroy driver instance, return interface instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write to a register
    fn write_register(&mut self, address: Registers, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [address.addr(), data];
        self.i2c.write(DEV_ADDR, &payload).map_err(Error::I2C)
    }

    /// Read from a register
    // REMOVE PUB LATER
    pub fn read_register(&mut self, address: Registers) -> Result<u8, Error<E>> {
        let mut data: [u8; 1] = [0];
        self.i2c
            .write_read(DEV_ADDR, &[address.addr()], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    /// Set specific bits using a bitmask
    fn set_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) == 0 {
            self.write_register(address, data | bitmask)            
        } else {
            Ok(())
        }
    }

    /// Clear specific bits using a bitmask
    fn clear_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), Error<E>> {
        let data = self.read_register(address)?;
        if (data & bitmask) != 0 {
            self.write_register(address, data & !bitmask)
        } else {
            Ok(())
        }
    }

    /// Check if specific bits are set.
    fn is_register_bit_flag_high(&mut self, address: Registers, bitmask: u8) -> Result<bool, Error<E>> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }


    /// Enable all axes
    pub fn enable_axes(&mut self) -> Result<(), Error<E>> {
        self.write_register(Registers::CFG_ODR, 0b1110_1111)?;
        Ok(())
    }


}



/// Output data rate and power mode selection (ODR). (see page 23)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ODR {    
    /// 1 Hz (not available in normal mode)
    _1Hz = 0b0000,
    /// 1.95 Hz (not available in normal mode)
    _1_95Hz = 0b0001,
    /// 3.90 Hz
    _3_90Hz = 0b0010,
    /// 7.81 Hz
    _7_81Hz = 0b0011,
    /// 15.63 Hz    
    _15_63Hz = 0b0100,
    /// 31.25Hz
    _31_25Hz = 0b0101,
    // 62.5Hz
    _62_5Hz = 0b0110,
    // 125Hz 
    _125Hz = 0b0111,
    // 250Hz 
    _250Hz = 0b1000,
    // 500Hz (not available in low power mode)
    _500Hz = 0b1001,
    // 1000Hz (not available in low power mode)
    _1000Hz = 0b1010,    
}

impl ODR {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Low power bandwidth. (see page 23)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum BW {        
    /// 1.95 Hz 
    _1_95Hz = 0b0010,
    /// 3.90 Hz
    _3_90Hz = 0b0011,
    /// 7.81 Hz
    _7_81Hz = 0b0100,
    /// 15.63 Hz    
    _15_63Hz = 0b0101,
    /// 31.25Hz
    _31_25Hz = 0b0110,
    // 62.5Hz
    _62_5Hz = 0b0111,
    // 125Hz 
    _125Hz = 0b1000,
    // 250Hz 
    _250Hz = 0b1001,
    // 500Hz
    _500Hz = 0b1010,
    
}

impl BW {
    pub fn value(self) -> u8 {
        self as u8
    }
}
 
/// Resolution of X/Y/Z axes. (see page 22)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum RES {
    /// 14-bit
    _14bit = 0b00,
    /// 12-bit
    _12bit = 0b01,
    /// 10-bit
    _10bit = 0b10,
    /// 8-bit
    _8bit = 0b11,
}

impl RES {
    pub fn value(self) -> u8 {
        (self as u8) << 2 // shifted into the correct position
    }
}

/// Acceleration range of X/Y/Z axes. (see page 23)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum RANGE {
    /// +/-2g
    _2g = 0b00,
    /// +/-4g
    _4g = 0b01,
    /// +/-8g
    _8g = 0b10,
    /// +/-16g
    _16g = 0b11,
}

impl RANGE {
    pub fn value(self) -> u8 {
        self as u8 // shifted into the correct position
    }
}



/// Interrupt active setting for the INT1 pin: active high (default) or active low
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_ACTIVE {
    /// Active high
    High,
    /// Active low
    Low,
}

impl INT_ACTIVE {
    pub fn status(self) -> bool {
        let status = match self {
            INT_ACTIVE::High => false,
            INT_ACTIVE::Low => true,
        };
        status
    }
}

/// Interrupt pad setting for INT1 pin: push-pull (default) or open-drain.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_PIN {
    /// Push-pull
    PushPull,
    /// Open drain
    OpenDrain,
}

impl INT_PIN {
    pub fn status(self) -> bool {
        let status = match self {
            INT_PIN::PushPull => false,
            INT_PIN::OpenDrain => true,
        };
        status
    }
}

 
/// Settings for various bit flags that can be Enabled (active) or Disabled (inactive)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FLAG {
    /// Enable (bit set)    
    Enable,
    /// Disable (bit cleared)
    Disable,
}

impl FLAG {
    pub fn status(self) -> bool {
        let status = match self {
            FLAG::Disable => false,
            FLAG::Enable => true,
        };
        status
    }
}

/// Settings for various bit flags regarding activity and tap detection, which can be either positive or negative
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum SIGN {
    /// Positive (bit set)
    Positive,
    /// Negaitive (bit cleared)
    Negative,
}

impl SIGN {
    pub fn status(self) -> bool {
        let status = match self {
            SIGN::Negative => false,
            SIGN::Positive => true,
        };
        status
    }
}

/// Settings for various bit flags regarding axis polarity and output swapping, which can be either positive or negative
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum POLARITY {
    /// Normal (not reversed) (bit cleared)
    Normal,
    /// Reversed/Swapped (bit set)
    Reversed,
}

impl POLARITY {
    pub fn status(self) -> bool {
        let status = match self {
            POLARITY::Normal => false,
            POLARITY::Reversed => true,
        };
        status
    }
}



/// Orientation mode of the x/y axes selection. (see page 22)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ORIENT_XY {
    /// Portrait upright
    PortraitUpright = 0b00,
    /// Portrait upside down
    PortraitUpsideDown = 0b01,
    /// Landscape left
    LandscapeLeft = 0b10,
    /// Landscape right
    LandscapeRight = 0b11,
}

impl ORIENT_XY {
    pub fn value(self) -> u8 {
        (self as u8) << 5 // shifted into the correct position, can be used directly
    }
}

/// Orientation mode of the z axis selection. (see page 22)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ORIENT_Z {
    /// Upward looking
    Upward = 0b00,
    /// Downward looking
    Downward = 0b01,
}

impl ORIENT_Z {
    pub fn value(self) -> u8 {
        self as u8 // shifted into the correct position, can be used directly
    }
}



/// Power mode (see page 23)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum PWR_MODE {
    /// Normal mode
    Normal = 0b00,
    /// Low power mode
    LowPower = 0b01,
    /// Suspend mode
    Suspend = 0b10,    
}

impl PWR_MODE {
    pub fn value(self) -> u8 {
        (self as u8)  << 6 // shifted into the correct position
    }
}


/// Interrupt latching (see page 25)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_LATCH {
 /// Non-latched
 NonLatched = 0b0000,
 /// temporary latched 250ms 0b0001
 TempLatch_250ms = 0b0001,
 /// temporary latched 500ms 0b0010
 TempLatch_500ms = 0b0010, 
 /// temporary latched 1s = 0b0011, 
 TempLatch_1s = 0b0011, 
 /// temporary latched 2s = 0b0100, 
 TempLatch_2s = 0b0100, 
 /// temporary latched 4s = 0b0101, 
 TempLatch_4s = 0b0101, 
 /// temporary latched 8s = 0b0110, 
 TempLatch_8s = 0b0110, 
 /// latched = 0b0111, 
 Latched = 0b0111,  
 /// temporary latched 1ms = 0b1001, 
 TempLatch_1ms = 0b1001, 
 /// temporary latched 1ms = 0b1010, 
 

// --- CHECK ADAFRUIT DRIVER! --

// TempLatch_1ms = 0b1010, 
 /// temporary latched 2ms = 0b1011, 
 TempLatch_2ms = 0b1011, 
 /// temporary latched 25ms = 0b1100, 
 TempLatch_25ms = 0b1100, 
 /// temporary latched 50ms = 0b1101, 
 TempLatch_50ms = 0b1101, 
 /// temporary latched 100ms = 0b1110, 
 TempLatch_100ms = 0b1110, 
}

impl INT_LATCH {
    pub fn value(self) -> u8 {
        self as u8 // shifted into the correct position
    }
}


/// Tap quiet duration. (see page 27)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum TAP_QUIET {
    /// Tap quiet duration 30ms
    _30ms = 0b00,
    /// Tap quiet duration 50ms
    _50ms =  0b01,
}

impl TAP_QUIET {
    pub fn value(self) -> u8 {
        (self as u8) << 7// shifted into the correct position, can be used directly
    }
}

/// Tap shock duration. (see page 27)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum TAP_SHOCK {
    /// Tap shock duration 30ms
    _50ms = 0b00,
    /// Tap shock duration 50ms
    _70ms =  0b01,
}

impl TAP_SHOCK {
    pub fn value(self) -> u8 {
        (self as u8) << 6 // shifted into the correct position, can be used directly
    }
}

/// Time window length for the second shock (see page 27)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum TAP_DUR {
    /// 50 ms
    _50ms = 0b000,
    /// 100 ms
    _100ms = 0b001,
    /// 150 ms
    _150ms = 0b010,
    /// 200 ms
    _200ms = 0b011,
    /// 250 ms
    _250ms = 0b100,
    /// 375 ms
    _375ms = 0b101,
    /// 500 ms
    _500ms = 0b110,
    /// 700 ms
    _700ms = 0b111,

}

impl TAP_DUR {
    pub fn value(self) -> u8 {
        self as u8 // shifted into the correct position, can be used directly
    }
}

/// Active interrupt threshold. (see page 26)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]

// --- ?????? --- CHECK ADAFRUIT DRIVER
pub enum THS_RANGE {    
    /*
    /// +/-2g
    _2g = 0b00,
    /// +/-4g
    _4g = 0b01,
    /// +/-8g
    _8g = 0b10,
    /// +/-16g
    _16g = 0b11,
    */
}

impl THS_RANGE {
    pub fn value(self) -> u8 {
        self as u8 // shifted into the correct position
    }
}

/// Tap threshold range. (see page 27)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]

// --- ?????? --- CHECK ADAFRUIT DRIVER
pub enum TAP_THS {    
    /*
    /// +/-2g
    _2g = 0b00,
    /// +/-4g
    _4g = 0b01,
    /// +/-8g
    _8g = 0b10,
    /// +/-16g
    _16g = 0b11,
    */
}

impl TAP_THS {
    pub fn value(self) -> u8 {
        self as u8 // shifted into the correct position
    }
}


/// Orientation interrupt blocking mode
pub enum ORIENT_BLOCK {
    /// No blocking 
    NoBlock = 0b00,
    /// Z-axis blocking 
    ZaxisBlock = 0b01,
    /// Z-axis blocking or slope in any axis > 0.2g 
    ZaxisBlockOrSlope = 0b10,
    
    // --- CHECK ADAFRUIT'S DRIVER ---

}

impl ORIENT_BLOCK {
    pub fn value(self) -> u8 {
        (self as u8) << 2 // shifted into position
    }
}
    
/// Orientation interrupt threshold setting
pub enum ORIENT_MODE {
    /// Symmetrical 
    Symmetrical = 0b00,
    /// High-asymmetrical 
    HighAsymmetrical = 0b01,
    /// Low-asymmetrical
    LowAsymmetrical = 0b10,
    
    // --- CHECK ADAFRUIT'S DRIVER ---

}

impl ORIENT_MODE {
    pub fn value(self) -> u8 {
        self as u8 // shifted into position
    }
}


/// Freefall mode. (see page 26)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FREEFALL_MODE {
    /// Single mode
    Single,
    /// Sum mode
    Sum,
}

impl FREEFALL_MODE {
    pub fn status(self) -> bool {
        let status = match self {
            FREEFALL_MODE::Single => false,
            FREEFALL_MODE::Sum => true,
        };
        status
    }
}