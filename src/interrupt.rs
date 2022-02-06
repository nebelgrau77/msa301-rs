//! Various functions related to interrupts
//!
//! TO DO: check if all functions related to the interrupts are covered

use super::*;

impl<I2C, E> MSA301<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{

    /// Enable/disable new data interrupt
    pub fn new_data_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
        match flag {
            FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET1, Bitmasks::NEW_DATA_INT_EN)?,
            FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET1, Bitmasks::NEW_DATA_INT_EN)?,
        }
        Ok(())
    }
    
    /// Enable/disable freefall interrupt
    pub fn freefall_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
       match flag {
           FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET1, Bitmasks::FREEFALL_INT_EN)?,
           FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET1, Bitmasks::FREEFALL_INT_EN)?,
       }
       Ok(())
    }

    /// Enable/disable orientation interrupt
    pub fn orient_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
        match flag {
            FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ORIENT_INT_EN)?,
            FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ORIENT_INT_EN)?,
        }
        Ok(())
    }

    /// Enable/disable single tap interrupt
    pub fn single_tap_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
        match flag {
            FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::S_TAP_INT_EN)?,
            FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::S_TAP_INT_EN)?,
        }
        Ok(())
    }

    /// Enable/disable double tap interrupt
    pub fn double_tap_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
        match flag {
            FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::D_TAP_INT_EN)?,
            FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::D_TAP_INT_EN)?,
        }
        Ok(())
    }

    /// Enable/disable active interrupt for X axis
    pub fn active_xaxis_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
        match flag {
            FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_X)?,
            FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_X)?,
        }
        Ok(())
    }

    /// Enable/disable active interrupt for Y axis
    pub fn active_yaxis_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
        match flag {
            FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Y)?,
            FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Y)?,
        }
        Ok(())
    }

    /// Enable/disable active interrupt for Z axis
    pub fn active_zaxis_int(&mut self, flag: FLAG) -> Result<(), Error<E>> {        
        match flag {
            FLAG::Disable => self.clear_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Z)?,
            FLAG::Enable => self.set_register_bit_flag(Registers::INT_SET0, Bitmasks::ACTIVE_INT_EN_Z)?,
        }
        Ok(())
    }

}


/*

/// Interrupt pin settings
#[derive(Debug)]
pub struct InterruptConfig {
    /// configure interrupt pin as active high or active low 
    pub active_high_or_low: INT_ACTIVE, 
    /// configure interrupt pin as  push-pull or open drain
    pub pushpull_or_opendrain: INT_PIN,
    /// configure data signal on the interrupt pin
    pub data_signal_config: INT_DRDY,
    /// enable FIFO full flag on interrupt pin
    pub enable_fifo_full: FLAG, 
    /// enable FIFO watermark flag on interrupt pin
    pub enable_fifo_fth: FLAG, 
    /// enable FIFO overrun flag on interrupt pin
    pub enable_fifo_overrun: FLAG,
    /// enable data ready signal on interrupt pin
    pub enable_data_ready: FLAG,
    /// enable latching interrupt request to INT_SOURCE register
    pub enable_latch_interrupt: FLAG,
    /// enable low pressure event on interrupt pin
    pub enable_low_event: FLAG,
    /// enable hihg pressure event on interrupt pin
    pub enable_high_event: FLAG,
    /// enable computing of differential pressure output
    pub enable_differential: FLAG,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        InterruptConfig {
            active_high_or_low: INT_ACTIVE::High,                // active high (CTRL_REG3)
            pushpull_or_opendrain: INT_PIN::PushPull,            // push-pull (CTRL_REG3)
            data_signal_config: INT_DRDY::DataSignal,            // data signal on INT_DRDY pin (CTRL_REG3)
            enable_fifo_full: FLAG::Disabled,                    // disabled (CTRL_REG3)
            enable_fifo_fth: FLAG::Disabled,                     // disabled (CTRL_REG3)
            enable_fifo_overrun: FLAG::Disabled,                 // disabled (CTRL_REG3)
            enable_data_ready: FLAG::Disabled,                   // disabled (CTRL_REG3)
            enable_latch_interrupt: FLAG::Disabled,              // interrupt request not latched (INTERRUPT_CFG)
            enable_low_event: FLAG::Disabled,                    // disable interrupt request on low pressure event (INTERRUPT_CFG)
            enable_high_event: FLAG::Disabled,                   // disable interrupt request on low pressure event (INTERRUPT_CFG)
            enable_differential: FLAG::Disabled,                 // disabled (CTRL_REG1)
        }
    }
}

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


 /*

#[derive(Debug)]
/// Contents of the INT_SOURCE register (interrupt active and differential pressure events flags)
pub struct IntStatus {
    pub interrupt_active: bool,
    pub diff_press_low: bool,
    pub diff_press_high: bool,    
}

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    /// Enable interrupts and configure the interrupt pin
    pub fn configure_interrupts(&mut self, 
                                // flag: bool, 
                                config: InterruptConfig)
                                 -> Result<(), T::Error> {        
        
        // write the whole CTRL_REG3 register                                    
        self.interface.write(Registers::CTRL_REG3.addr(), config.int_ctrl_reg3())?;                
        
        // get the contents of INTERRUPT_CFG and combine it with the bits to be set
        let mut reg_data = [0u8;1];
        self.read_register(Registers::INTERRUPT_CFG)?;        
                
        let mut interrupt_cfg = config.int_interrupt_cfg();
               
        let mut data: u8 = reg_data[0] & !0b00001111;

        data = data | interrupt_cfg;

        self.interface.write(Registers::INTERRUPT_CFG.addr(), data)?;        
        
        Ok(())
    }
    
    
 /// Get all the flags from the INT_SOURCE register (NOTE: INT_SOURCE register is cleared by reading it)
 pub fn get_int_status(&mut self) -> Result<IntStatus, T::Error> {        
                
    let reg_value = self.read_register(Registers::INT_SOURCE)?;

    let status = IntStatus {
        /// Has any interrupt event been generated?
        interrupt_active: match reg_value & Bitmasks::IA {
            0 => false,
            _ => true,
        },
        /// Has low differential pressure event been generated?
        diff_press_low: match reg_value & Bitmasks::PL {
            0 => false,
            _ => true,
        },
        /// Has high differential pressure event been generated?
        diff_press_high: match reg_value & Bitmasks::PH {
            0 => false,
            _ => true,
        },           
    };
    Ok(status)
 }

}
 */    
