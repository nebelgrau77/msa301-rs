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
                            AccelConfig {                                
                                ..Default::default()
                                /*
                                enable_axes: (true, true, true),
                                powermode: PowerMode::Normal,
                                datarate: DataRate::_125Hz,
                                bandwidth: BandWidth::_62_5Hz,
                                resolution: Res::_14bit,
                                range: Range::_2g,
                                 */
                                });

    msa301.init_sensor(AccelConfig{..Default::default()}).unwrap();

    let cfgodr = msa301.read_register(Registers::CFG_ODR).unwrap();
    let resrange = msa301.read_register(Registers::RES_RANGE).unwrap();
    let pwrbw = msa301.read_register(Registers::PWR_BW).unwrap();
    
    println!("CFG_ODR: {:08b}\nRES_RANGE: {:08b}\nPWR_BW: {:08b}\n", 
                    cfgodr, resrange, pwrbw);
    
    /*
    msa301.set_datarate(DataRate::_125Hz).unwrap();
    msa301.set_bandwidth(BandWidth::_62_5Hz).unwrap();
    
    let cfgodr = msa301.read_register(Registers::CFG_ODR).unwrap();
    let resrange = msa301.read_register(Registers::RES_RANGE).unwrap();
    let pwrbw = msa301.read_register(Registers::PWR_BW).unwrap();
    
    println!("CFG_ODR: {:08b}\nRES_RANGE: {:08b}\nPWR_BW: {:08b}\n", 
                    cfgodr, resrange, pwrbw);

 */
    let (x,y,z) = msa301.read_accel().unwrap();
    println!("x {}, y {}, z {}\r\n", x, y, z);      

    
    loop {
        let (x,y,z) = msa301.read_accel().unwrap();
        //println!("x {}\r\n", x);        
        //println!("y {}\r\n", y);        
        println!("z {}\r\n", z);        
    }
    
}
