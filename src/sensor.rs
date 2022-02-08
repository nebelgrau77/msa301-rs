//! Functions related to sensor measurements: reading value or status, setting offset and reference
//!
//! TO DO: add reference pressure setting

use super::*;

/*

#[derive(Debug)]
/// Contents of the STATUS register (pressure and temperature overrun and data availability flags)
pub struct DataStatus {
    pub temp_overrun: bool,
    pub press_overrun: bool,
    pub temp_available: bool,
    pub press_available: bool,
}
 */

impl<I2C, E> MSA301<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{

    /// Read the device ID ("who am I")
    pub fn get_device_id(&mut self) -> Result<u8, Error<E>> {
        //let mut data = [0u8; 1];
        //self.interface.read(Registers::PART_ID.addr(), &mut data)?;        
        let whoami = self.read_register(Registers::PART_ID)?;
        Ok(whoami)
    }


    /// read raw sensor values
    pub fn read_accel_raw(&mut self) -> Result<(u8, u8, u8, u8, u8, u8), Error<E>> {
        let mut data = [0_u8;6];
        self.i2c.write_read(DEV_ADDR, &[Registers::XAXIS_L.addr()], &mut data)
        .map_err(Error::I2C)
        .and(Ok((data[0],data[1],data[2],data[3],data[4],data[5])))
    }

    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let (x_lo, x_hi, y_lo, y_hi, z_lo, z_hi) = self.read_accel_raw()?;

        // scale can be retrieved using the .sensitivity method
        // but the Range has to be stored somewhere, in settings
        // otherwise there must be a get_range function to read the bytes
        // corresponding to the Range setting

        let scale: u16 = 1024; // some arbitrary scale, to be replaced later

        let mut raw_x: u16 = (x_hi << 8) + x_lo as u16; 
        raw_x = raw_x >> 2;
        let x: f32 = raw_x as f32 / scale as f32

        let mut raw_y: u16 = (y_hi << 8) + y_lo as u16; 
        raw_y = raw_y >> 2;
        let y: f32 = raw_y as f32 / scale as f32;

        let mut raw_z: u16 = (z_hi << 8) + z_lo as u16; 
        raw_z = raw_z >> 2;
        let z: f32 = raw_z as f32 / scale as f32;

        Ok((x,y,z))

    }


    /*

     /// Calculated pressure reading in hPa
     pub fn read_pressure(&mut self) -> Result<f32, T::Error> {
        let mut data = [0u8; 3];
        self.interface.read(
            Registers::PRESS_OUT_XL.addr(),
            &mut data,
        )?;
        let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
        let pressure = (p as f32) / PRESS_SCALE; // no need to take care of negative values
        Ok(pressure)
    }
 */
    
    /*

    /// Read threshold value for pressure interrupt generation
    pub fn read_threshold(&mut self) -> Result<i16, T::Error> {
        let mut data = [0u8; 2];
        self.interface.read(Registers::THS_P_L.addr(), &mut data)?;
        let o: i16 = (data[1] as i16) << 8 | (data[0] as i16);
        Ok(o)
    }
 
    /// Set the pressure offset value (VALUE IN hPA!)
    pub fn set_threshold(&mut self, threshold: u16) -> Result<(), T::Error> {
        let mut payload = [0u8; 2];
        let threshold = threshold * 16;

        payload[0] = (threshold & 0xff) as u8; // lower byte
        payload[1] = (threshold >> 8) as u8; // upper byte

        self.interface.write(Registers::THS_P_L.addr(), payload[0])?;
        self.interface.write(Registers::THS_P_H.addr(), payload[1])?;

        Ok(())
    }

     */

     /*

    /// Get all the flags from the STATUS_REG register
    pub fn get_data_status(&mut self) -> Result<DataStatus, T::Error> {
        // TO DO: use this value for reading all the bitflags in one go
        // use bitmasks
        let reg_value = self.read_register(Registers::STATUS)?;

        let status = DataStatus {
            /// Has new pressure data overwritten the previous one?
            press_overrun: match reg_value & Bitmasks::P_OR {
                0 => false,
                _ => true,
            },
            /// Has new temperature data overwritten the previous one?
            temp_overrun: match reg_value & Bitmasks::T_OR {
                0 => false,
                _ => true,
            },
            /// Is new pressure data available?
            press_available: match reg_value & Bitmasks::P_DA {
                0 => false,
                _ => true,
            },
            /// Is new temperature data available?
            temp_available: match reg_value & Bitmasks::T_DA {
                0 => false,
                _ => true,
            },
        };

        Ok(status)
    }
    */

    /*
    /// Triggers the one-shot mode, and a new acquisition starts when it is required.
    /// Enabling this mode is possible only if the device was previously in power-down mode.
    /// Once the acquisition is completed and the output registers updated,
    /// the device automatically enters in power-down mode. ONE_SHOT bit self-clears itself.
    pub fn one_shot(&mut self) -> Result<(), T::Error> {
        self.set_datarate(ODR::PowerDown)?; // make sure that Power down/one shot mode is enabled
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
        Ok(())
    }
    */

}
