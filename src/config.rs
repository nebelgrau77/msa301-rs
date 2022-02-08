//! Various functions related to configuration
//!
//! TO DO: 
//! * check if all the functions are implemented
//! * how to limit datarate/bandwidth settings for the normal and low power mode?//! 
//! 

use super::*;

// --- DEFAULT CONFIGURATION ---

// the default configuration in the Adafruit driver has:
// - all axes enabled (default setting enabled)
// - power mode normal (default setting suspended mode)
// - data rate 500 Hz (default setting 1000 Hz)
// - bandwidth 250 Hz (default setting 500 Hz)
// - range 4G (default range is 2g)
// - resolution 14 bit  (default resolution)

/// Accelerometer settings to configure the sensor
#[derive(Debug)]
pub struct AccelConfig {
    /// Axes X,Y,Z enabled
    pub enable_axes: (bool, bool, bool),
    /// Set power mode
    pub powermode: PowerMode,
    /// Set output datarate    
    pub datarate: DataRate,
    /// Set bandwidth
    pub bandwidth: BandWidth,    
    /// Set resolution in bits
    pub resolution: Res,
    /// Set full scale range
    pub range: Range,
}

impl Default for AccelConfig {
    fn default() -> Self {
        AccelConfig {
            enable_axes: (true, true, true),
            powermode: PowerMode::Normal,
            datarate: DataRate::_500Hz,
            bandwidth: BandWidth::_250Hz,
            resolution: Res::_14bit,
            range: Range::_4g,
        }
    }
}

impl AccelConfig {
    // do I need this or it's enough to call various setting functions in the `init()` function?
    /// Values to be written to the RES_RANGE register
    pub (crate) fn res_range(&self) -> u8 {
        let mut data: u8 = 0;
        data |= self.range.value();
        data |= self.resolution.value();
        data
    }

    /// Values to be written to the CFG_ODR register
    /// Output data rate and enabling X/Y/Z axes
    pub (crate) fn cfg_odr(&self) -> u8 {
        let mut data: u8 = 0b1110_0000; // if bits 7:5 are set, all axes are DISABLED
        let (x,y,z) = self.enable_axes;
        if x {
            data = data & !0b1000_0000 // clear bit 7 to enable axis X
        }
        if y {
            data = data & !0b0100_0000 // clear bit 6 to enable axis Y
        }
        if z {
            data = data & !0b0010_0000 // clear bit 5 to enable axis Z
        }

        data |= self.datarate.value();
        data

    }

    /// Values to be written to the PWR_BW register
    /// Bandwidth and power mode settings
    pub (crate) fn pwr_bw(&self) -> u8 {
        let mut data: u8 = 0;
        data |= self.bandwidth.value();
        data |= self.powermode.value();
        data
    }
}


// === ARE THESE FUNCTIONS NECESSARY? === 

impl<I2C, E> MSA301<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Set output data rate
    pub fn set_datarate(&mut self, odr: DataRate) -> Result<(), Error<E>> {
        let reg = self.read_register(Registers::CFG_ODR)?;

        let mut data = reg & !Bitmasks::ODR_MASK;
        data |= odr.value();

        self.write_register(Registers::CFG_ODR, data)?;

        Ok(())
    }

    /// Set bandwidth
    pub fn set_bandwidth(&mut self, bandwidth: BandWidth) -> Result<(), Error<E>> {
        let reg = self.read_register(Registers::PWR_BW)?;     
        let mut data = reg & !Bitmasks::BW_MASK;
        data |= bandwidth.value();      
        self.write_register(Registers::PWR_BW, data)?;      
        Ok(())
    }

    /// Set power mode
    pub fn set_power_mode(&mut self, powermode: PowerMode) -> Result<(), Error<E>> {
        let reg = self.read_register(Registers::PWR_BW)?;     
        let mut data = reg & !Bitmasks::PWR_MASK;
        data |= powermode.value();      
        self.write_register(Registers::PWR_BW, data)?;      
        Ok(())
    }

    /// Set resolution in bits
    pub fn set_resolution(&mut self, resolution: Res) -> Result<(), Error<E>> {
        let reg = self.read_register(Registers::RES_RANGE)?;     
        let mut data = reg & !Bitmasks::RESOLUTION;
        data |= resolution.value();      
        self.write_register(Registers::RES_RANGE, data)?;      
        Ok(())
    }

    /// Set acceleration range (full scale)
    pub fn set_range(&mut self, range: Range) -> Result<(), Error<E>> {
        let reg = self.read_register(Registers::RES_RANGE)?;     
        let mut data = reg & !Bitmasks::FS;
        data |= range.value();      
        self.write_register(Registers::RES_RANGE, data)?;      
        Ok(())
    }
   
}
