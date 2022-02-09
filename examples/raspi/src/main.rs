//! talking to the MSA301 module over I2C on Raspberry Pi
//! 
//! readings look strange, maybe need calibration?
//! should the gravity be taken into account?

use rppal::i2c::I2c;

use msa301::*;
use msa301::config::AccelConfig;
use msa301::register::Registers;

fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // create a new driver instance with the I2C interface and configuration settings      
    let mut msa301 = MSA301::new(i2c,
                            AccelConfig {..Default::default()});
    
    loop {
        let (x,y,z) = msa301.read_accel().unwrap();
        println!("x {}, y {}, z {}\r\n", x, y, z);        
    }
       
}
