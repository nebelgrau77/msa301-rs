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

pub mod interface;
use interface::Interface;

/// Sensor's ID
// const PARTID: u8 = 0x13; // decimal value 19

/// Holds the driver instance with the selected interface
pub struct MSA301<T> {
    interface: T,
}

impl<T, E> MSA301<T>
where
    T: Interface<Error = E>,
{
    /// Create a new instance of the LPS25HB driver.
    pub fn new(interface: T) -> Self {
        MSA301 { interface }
    }

    /// Destroy driver instance, return interface instance.
    pub fn destroy(self) -> T {
        self.interface
    }

    /// Read a byte from the given register.
    fn read_register(&mut self, address: Registers) -> Result<u8, T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(address.addr(), &mut reg_data)?;
        Ok(reg_data[0])
    }

    /// Clear selected bits using a bitmask
    fn clear_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
        let mut reg_data = [0u8; 1];
        self.interface.read(address.addr(), &mut reg_data)?;
        let payload: u8 = reg_data[0] & !bitmask;
        self.interface.write(address.addr(), payload)?;
        Ok(())
    }

    /// Set selected bits using a bitmask
    fn set_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
        let mut reg_data = [0u8; 1];
        self.interface.read(address.addr(), &mut reg_data)?;
        let payload: u8 = reg_data[0] | bitmask;
        self.interface.write(address.addr(), payload)?;
        Ok(())
    }

    /// Check if specific bits are set.
    fn is_register_bit_flag_high(
        &mut self,
        address: Registers,
        bitmask: u8,
    ) -> Result<bool, T::Error> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }
}



/// Output data rate and power mode selection (ODR). (Refer to page 23)
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

/// Low power bandwidth. (Refer to page 23)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ODR {        
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

impl ODR {
    pub fn value(self) -> u8 {
        self as u8
    }
}
 

/*

/// FIFO mode selection. (Refer to Table 20)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFO_MODE {
    /// Bypass mode
    Bypass = 0b000,
    /// FIFO mode
    FIFO = 0b001,
    /// Stream mode
    Stream = 0b010,
    /// Stream-to-FIFO mode
    Stream_to_FIFO = 0b011,
    /// Bypass-to-stream mode
    Bypass_to_stream = 0b100,
    /// Dynamic-stream mode
    Dynamic_Stream = 0b110,
    /// Bypass-to-FIFO mode
    Bypass_to_FIFO = 0b111,
}

impl FIFO_MODE {
    pub fn value(self) -> u8 {
        (self as u8) << 5 // shifted into the correct position, can be used directly
    }
}

 */

/*

/// INT_DRDY pin configuration. (Refer to Table 19)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_DRDY {
    /// Data signal (see CTRL_REG4)
    DataSignal = 0b00,
    /// Pressure high
    P_high = 0b01,
    /// Pressure low
    P_low = 0b10,
    /// Pressure low or high
    P_low_or_high = 0b011,
}

impl INT_DRDY {
    pub fn value(self) -> u8 {
        self as u8 // no need to shift, bits 0:1 (INT_S)
    }
}
*/
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

 



/// Settings for various bit flags that can be Active or Inactive
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FLAG {
    
    // -- WOULD IT BE BETTER TO KEEP ENABLE/DISABLE? --
    
    /// Active (bit set)    
    Active,
    /// Inactive (bit cleared)
    Inactive,
}

impl FLAG {
    pub fn status(self) -> bool {
        let status = match self {
            FLAG::Inactive => false,
            FLAG::Active => true,
        };
        status
    }
}

/// Settings for various bit flags regarding activity and tap detection, whic can be either positive or negative
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum POLARITY {
    /// Positive (bit set)
    Positive,
    /// Negaitive (bit cleared)
    Negative,
}

impl POLARITY {
    pub fn status(self) -> bool {
        let status = match self {
            POLARITY::Negative => false,
            POLARITY::Positive => true,
        };
        status
    }
}

/*

/// FIFO on/off
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFO_ON {
    /// Enabled (bit set)
    Enabled,
    /// Disabled (bit cleared)
    Disabled,
}

impl FIFO_ON {
    pub fn status(self) -> bool {
        let status = match self {
            FIFO_ON::Disabled => false,
            FIFO_ON::Enabled => true,
        };
        status
    }
}

 */

// --- THESE VALUES ARE READ ONLY -> WILL NEED MATCHING BITS TO ENUM FIELDS ---

/// Orientation mode of the x/y axes selection. (Refer to page 22)
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

/// Orientation mode of the z axis selection. (Refer to page 22)
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

/// Resolution of X/Y/Z axes. (Refer to page 22)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum RES {
    /// 14-bit
    _14bit = 0b00,
    /// 12-bit
    _12bit = 0b01,
    /// 10-bit
    _10bit = 0b00,
    /// 8-bit
    _8bit = 0b00,
}

impl RES {
    pub fn value(self) -> u8 {
        (self as u8) << 2 // shifted into the correct position
    }
}

/// Acceleration range of X/Y/Z axes. (Refer to page 23)
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

/// Power mode (Refer to page 23)
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

/*
0000: non-latched
0001: tempoary latched 250ms
0010: tempoary latched 500ms
0011: tempoary latched 1s
0100: tempoary latched 2s
0101: tempoary latched 4s
0110: tempoary latched 8s
0111: latched
1000: non-latched
1001: tempoary latched 1ms
1010: tempoary latched 1ms
1011: tempoary latched 2ms
1100: tempoary latched 25ms
1101: tempoary latched 50ms
1110: tempoary latched 100m
*/