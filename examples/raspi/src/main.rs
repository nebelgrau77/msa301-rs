//! talking to the MSA301 module over I2C on Raspberry Pi
//! 
//! ID retrieved correctly (decimal 19/ 0x13)

use rppal::i2c::I2c;

use msa301::*;


fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // create a new driver instance with the I2C interface    
    let mut msa301 = MSA301::new(i2c);

    let id = msa301.get_device_id().unwrap();

    println!("Device ID: {}\n", id);

}
