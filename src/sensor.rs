//! Functions related to sensor measurements: reading value or status, setting offset and reference
//!
//! TO DO: 
//! * check if all the functions are implemented
//! * should the readings be returned as a struct?

use super::*;

impl<I2C, E> MSA301<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Read the device ID ("who am I"). Returns decimal value 19.
    pub fn get_device_id(&mut self) -> Result<u8, Error<E>> {
        let whoami = self.read_register(Registers::PART_ID)?;
        Ok(whoami)
    }
    
    /// Read raw sensor values
    fn read_accel_raw(&mut self) -> Result<([u8;6]), Error<E>> {
        let mut data = [0_u8;6];
        self.i2c.write_read(DEV_ADDR, &[Registers::XAXIS_L.addr()], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data))
        }

    /// Read the accelerometer data as a tuple, 
    /// correctly scaled according to the selected range.
    /// 
    /// ```rust
    /// let (x,y,z) = msa301.read_accel().unwrap();            
    /// println!("x: {}, y: {}, z: {}\r\n", x,y, z);  
    /// ```
    /// 
    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let raw_data = self.read_accel_raw()?;

        let scale = self.config.range.sensitivity();

        let mut raw_x = (raw_data[1] as i16) << 8 | (raw_data[0] as i16);
        raw_x = raw_x >> 2;
        let x = (raw_x as f32) / scale;
        let mut raw_y = (raw_data[3] as i16) << 8 | (raw_data[2] as i16);
        raw_y = raw_y >> 2;
        let y = (raw_y as f32) / scale;
        let mut raw_z = (raw_data[5] as i16) << 8 | (raw_data[4] as i16);
        raw_z = raw_z >> 2;
        let z = (raw_z as f32) / scale;

        Ok((x,y,z))

    }

    /*

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

   

}
