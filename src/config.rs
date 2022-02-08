//! Various functions related to configuration
//!
//! TO DO: check if all the functions are implemented
//! 

use super::*;

/*
impl<T, E> MSA301<T>
where
    T: Interface<Error = E>,
{
 */


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
    fn res_range(&self) -> u8 {
        let mut data: u8 = 0;
        data |= self.range.value();
        data |= self.resolution.value();
        data
    }

    fn cfg_odr(&self) -> u8 {
        let mut data: u8 = 0b1110_0000;
        let (x,y,z) = self.enable_axes;
        if x {
            data = data & !0b1000_0000 // clear bit 7
        }
        if y {
            data = data & !0b0100_0000 // clear bit 6
        }
        if z {
            data = data & !0b0010_0000 // clear bit 5
        }

        data |= self.datarate.value();

        data

    }

    fn pwr_bw(&self) -> u8 {
        let mut data: u8 = 0;
        data |= self.bandwidth.value();
        data |= self.powermode.value();
    }






}


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
    pub fn set_resolution(&self, resolution: Res) -> Result<(), Error<>> {
        let reg = self.read_register(Registers::RES_RANGE)?;     
        let mut data = reg & !Bitmasks::RESOLUTION;
        data |= resolution.value();      
        self.write_register(Registers::RES_RANGE, data)?;      
        Ok(())
    }

    /// Set acceleration range (full scale)
    pub fn set_range(&self, range: Range) -> Result<(), Error<>> {
        let reg = self.read_register(Registers::RES_RANGE)?;     
        let mut data = reg & !Bitmasks::FS;
        data |= range.value();      
        self.write_register(Registers::RES_RANGE, data)?;      
        Ok(())
    }

    /*
    /// Set output data rate        
    
    pub fn set_datarate(&mut self, odr: DataRate) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface
            .read(Registers::CTRL_REG1.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::DataRate_MASK;
        payload |= odr.value();
        self.interface.write(Registers::CTRL_REG1.addr(), payload)?;
        Ok(())
    }
     */
    
    
    /*

    /// Register address automatically incremented during a multiple byte access with a serial interface (I2C or SPI).
    /// Default value: enabled
    pub fn address_incrementing(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::IF_ADD_INC),
            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::IF_ADD_INC),
        }
    }

     */ 

    /*

    /// Reboot. Refreshes the content of the internal registers stored in the Flash memory block.
    /// At device power-up the content of the Flash memory block is transferred to the internal registers
    /// related to the trimming functions to allow correct behavior of the device itself.
    /// If for any reason the content of the trimming registers is modified,
    /// it is sufficient to use this bit to restore the correct values.
    /// At the end of the boot process the BOOT bit is set again to ‘0’ by hardware.
    /// The BOOT bit takes effect after one ODR clock cycle.
    pub fn reboot(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::BOOT)
    }

    /// Is reboot phase running?
    pub fn reboot_running(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::BOOT_STATUS)
    }

    /// Run software reset (resets the device to the power-on configuration, takes 4 usec)
    pub fn software_reset(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::SWRESET)
    }

     */

    /*

    // SWITCHING INTO POWER-DOWN COULD BE ADDED TO THIS FUNCTION
    /// Enable low-power mode (must be done only with the device in power-down mode)
    pub fn enable_low_power(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::RES_CONF, Bitmasks::LC_EN)
    }

    // LOWPASS FILTER ENABLING AND CONFIGURING COULD BE MOVED TOGETHER

    /// Enable and configure low-pass filter on pressure data in Continuous mode
    pub fn lowpass_filter(&mut self, enable: bool, configure: bool) -> Result<(), T::Error> {
        match enable {
            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
        }?;
        match configure {
            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
        }?;
        Ok(())
    }
   
    /// Reset low-pass filter.  If the LPFP is active, in order to avoid the transitory phase,
    /// the filter can be reset by reading this register before generating pressure measurements.
    pub fn lowpass_filter_reset(&mut self) -> Result<(), T::Error> {
        let mut _data = [0u8; 1];
        self.interface
            .read(Registers::LPFP_RES.addr(), &mut _data)?;
        Ok(())
    }

     */
}
