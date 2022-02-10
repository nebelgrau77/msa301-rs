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
    /// Read the device ID ("who am I")
    pub fn get_device_id(&mut self) -> Result<u8, Error<E>> {
        let whoami = self.read_register(Registers::PART_ID)?;
        Ok(whoami)
    }

    // === MAKE THIS FUNCTION PRIVATE
    /// Read raw sensor values
    pub fn read_accel_raw(&mut self) -> Result<(u8, u8, u8, u8, u8, u8), Error<E>> {
        let mut data = [0_u8;6];
        self.i2c.write_read(DEV_ADDR, &[Registers::XAXIS_L.addr()], &mut data)
            .map_err(Error::I2C)
            .and(Ok((data[0],data[1],data[2],data[3],data[4],data[5])))


        // let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);

    }

    /// Read the accelerometer data correctly scaled
    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), Error<E>> {
        let (x_lo, x_hi, y_lo, y_hi, z_lo, z_hi) = self.read_accel_raw()?;

        // == SCALE MUST BE U16!! ==
        let scale = self.config.range.sensitivity();

        let mut raw_x: i16 = (x_hi as i16) << 8 | x_lo as i16; 
        raw_x = raw_x >> 2;        
        let x: f32 = raw_x as f32 / scale;

        let mut raw_y: i16 = (y_hi as i16) << 8 | y_lo as i16; 
        raw_y = raw_y >> 2;
        let y: f32 = raw_y as f32 / scale;

        let mut raw_z: i16 = (z_hi as i16) << 8 | z_lo as i16; 
        raw_z = raw_z >> 2;
        let z: f32 = raw_z as f32 / scale;

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
