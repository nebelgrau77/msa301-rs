//! Various functions related to interrupts
//!
//! TO DO: 
//! * check if all functions related to the interrupts are covered
//! * is an interrupt config struct necessary?
//! * is a status struct necessary?

use super::*;

/// Motion interrupts status
#[derive(Debug)]
pub struct InterruptStatus {
    pub orientation: bool,
    pub single_tap: bool,
    pub double_tap: bool,
    pub active: bool,
    pub freefall: bool,
}

/// Motion interrupts status
#[derive(Debug)]
pub struct InterruptConfig {
    pub pin_output: IntPin,
    pub pin_active: IntActive,
    pub orientation: Flag,
    pub single_tap: Flag,
    pub double_tap: Flag,
    pub active: Flag,
    pub freefall: Flag,
    pub new_data: Flag,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        InterruptConfig {
            pin_output: IntPin::PushPull,
            pin_active: IntActive::High,
            orientation: Flag::Disable,
            single_tap: Flag::Disable,
            double_tap: Flag::Disable,
            active: Flag::Disable,
            freefall: Flag::Disable,
            new_data: Flag::Disable,
        }
    }    
}

impl<I2C, E> MSA301<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Enable/disable new data interrupt
    /// 
    /// '''rust
    /// msa301.new_data_int(Flag::Enable).unwrap();
    /// ```
    /// 
    pub fn new_data_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
        match flag {
            Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET1, Bitmasks::NEW_DATA_INT_EN)?,
            Flag::Enable => self.set_register_bit_flag(Registers::INT_SET1, Bitmasks::NEW_DATA_INT_EN)?,
        }
        Ok(())
    }
    
    /// Enable/disable freefall interrupt
    /// 
    /// '''rust
    /// msa301.freefall_int(Flag::Enable).unwrap();
    /// ```
    /// 
    pub fn freefall_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
       match flag {
           Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET1, Bitmasks::FREEFALL_INT_EN)?,
           Flag::Enable => self.set_register_bit_flag(Registers::INT_SET1, Bitmasks::FREEFALL_INT_EN)?,
       }
       Ok(())
    }

    /// Enable/disable orientation interrupt
    /// 
    /// '''rust
    /// msa301.orient_int(Flag::Enable).unwrap();
    /// ```
    /// 
    pub fn orient_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
        match flag {
            Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ORIENT_INT_EN)?,
            Flag::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ORIENT_INT_EN)?,
        }
        Ok(())
    }

    /// Enable/disable single tap interrupt
    ///  
    /// '''rust
    /// msa301.single_tap_int(Flag::Enable).unwrap();
    /// ```
    /// 
    pub fn single_tap_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
        match flag {
            Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::S_TAP_INT_EN)?,
            Flag::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::S_TAP_INT_EN)?,
        }
        Ok(())
    }

    /// Enable/disable double tap interrupt
    ///  
    /// '''rust
    /// msa301.double_tap_int(Flag::Enable).unwrap();
    /// ```
    /// 
    pub fn double_tap_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
        match flag {
            Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::D_TAP_INT_EN)?,
            Flag::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::D_TAP_INT_EN)?,
        }
        Ok(())
    }

    /// Enable/disable active interrupt for X axis
    ///  
    /// '''rust
    /// msa301.active_xaxis_int(Flag::Enable).unwrap();
    /// ```
    /// 
    pub fn active_xaxis_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
        match flag {
            Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_X)?,
            Flag::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_X)?,
        }
        Ok(())
    }

    /// Enable/disable active interrupt for Y axis///
    pub fn active_yaxis_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
        match flag {
            Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Y)?,
            Flag::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Y)?,
        }
        Ok(())
    }

    /// Enable/disable active interrupt for Z axis
    pub fn active_zaxis_int(&mut self, flag: Flag) -> Result<(), Error<E>> {        
        match flag {
            Flag::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Z)?,
            Flag::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Z)?,
        }
        Ok(())
    }


    /// Configures the pin
    pub fn config_int_pin(&mut self, output: IntPin, active: IntActive) -> Result<(), Error<E>> {
        let mut data: u8 = 0;

        let pin_out: u8 = match output {
            IntPin::OpenDrain => 1 << 1, // shifted into position
            IntPin::PushPull => 0,
        };

        let pin_active: u8 = match active {
            IntActive::High => 1,
            IntActive::Low => 0,
        };

        data = data + pin_out + pin_active;

        self.write_register(Registers::INT_CFG, data)?;

        Ok(())
    }

    /// Configure interrupt latching
    pub fn config_int_latch(&mut self, reset: bool, setting: IntLatch) -> Result<(), Error<E>> {
        
        // should RESET be separated? probably!

        let reg = self.read_register(Registers::INT_LATCH)?;
        
        let mut data: u8 = 0;

        data = match reset {
            true => 0b1000_0000,
            false => 0b0000_0000,
        };

        // let mut data = reg & !Bitmasks::LATCH_INT;

        data |= setting.value();

        Ok(())

    }

    /// Get motion interrupts status
    pub fn motion_int_status(&mut self) -> Result<InterruptStatus, Error<E>> {
        let data = self.read_register(Registers::MOTION_INT)?;
        let mask: u8 = 0b1111_1110;
        let status = InterruptStatus {
                        
            orientation: match (data >> 6) &!mask {
                1 => true,
                _ => false,                
            },
            single_tap: match (data >> 5) &!mask  {
                1 => true,
                _ => false,
            },
            double_tap: match (data >> 4) &!mask  {
                1 => true,
                _ => false,
            },
            active: match (data >> 2) &!mask  {
                1 => true,
                _ => false,
            },
            freefall: match data &!mask  {
                1 => true,
                _ => false,
            },
        };
        Ok(status)
    }


        

    /// Check if new data available
    pub fn is_new_data(&mut self) -> Result<bool, Error<E>> {
        let data = self.read_register(Registers::DATA_INT)?;
        match data & 0x1 {            
            1 => Ok(true),
            _ => Ok(false)            
        }
    }

        /*

    /// TEST: route new data interrupt to pin INT
    pub fn new_data_pin(&mut self, setting: Flag) -> Result<(), Error<E>> {
                
        let val = match setting {
            Flag::Enable => 0x01,
            Flag::Disable => 0x00,
        };

        self.write_register(Registers::INT_MAP1, val)?;

        Ok(())

    }

    */


}


/*

impl InterruptConfig {
    /// Returns values to be written to CTRL_REG3, CTRL_REG4 and INTERRUPT_CFG:
    fn int_ctrl_reg3(&self) -> u8 {
        let mut data = 0u8;
        if self.active_high_or_low.status() {
            data |= 1 << 7;
        }
        if self.pushpull_or_opendrain.status() {
            data |= 1 << 6;
        }
        if self.enable_fifo_full.status() {
            data |= 1 << 5;
        }
        if self.enable_fifo_fth.status() {
            data |= 1 << 4;
        }
        if self.enable_fifo_overrun.status() {
            data |= 1 << 3;
        }
        if self.enable_data_ready.status() {
            data |= 1 << 2;
        }        
        data |= self.data_signal_config.value();
        data
    }    
    fn int_interrupt_cfg(&self) -> u8 {
        
        let mut data = 0u8;

        if self.enable_differential.status() {
            data |=1 << 3;
        }
        if self.enable_latch_interrupt.status() {
            data |= 1 << 2;
        }
        if self.enable_low_event.status() {
            data |= 1 << 1;
        }
        if self.enable_high_event.status() {
            data |= 1;
        }
        data // this must be OR'ed with the content of the INTERRUPT_CFG
    }
}

 */


