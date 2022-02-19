//! talking to the MSA301 module over I2C on Raspberry Pi
//! 

use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;

use msa301::*;
use msa301::config::AccelConfig;
use msa301::register::Registers;


fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // create a new driver with a default configuration
    let mut msa301 = MSA301::new(i2c, 
                                
                                AccelConfig{
                                    datarate: DataRate::_125Hz,
                                    bandwidth: BandWidth::_62_5Hz,
                                    ..Default::default()},
                                ).unwrap();
         
    //msa301.init().unwrap();
    
    thread::sleep(Duration::from_millis(500));

    let cfgodr = msa301.read_register(Registers::CFG_ODR).unwrap();
    let resrange = msa301.read_register(Registers::RES_RANGE).unwrap();
    let pwrbw = msa301.read_register(Registers::PWR_BW).unwrap();
    
    println!("CFG_ODR: {:08b}\nRES_RANGE: {:08b}\nPWR_BW: {:08b}\n", 
                    cfgodr, resrange, pwrbw);
    
    println!("Scale factor: {}", msa301.get_scale().unwrap());

    println!("{:?}", msa301.get_config().unwrap());

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z);        

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z);    

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z);    

    thread::sleep(Duration::from_millis(500));

    println!("change range to 16g");

    msa301.set_range(Range::_16g).unwrap();

    let resrange = msa301.read_register(Registers::RES_RANGE).unwrap();

    println!("current config: {:?}", msa301.get_config().unwrap());

    println!("Scale factor: {}", msa301.get_scale().unwrap());

    thread::sleep(Duration::from_millis(500));

    //msa301.set_bandwidth(BandWidth::_250Hz).unwrap();
    //msa301.set_datarate(DataRate::_500Hz).unwrap();

    let cfgodr = msa301.read_register(Registers::CFG_ODR).unwrap();                                
    println!("CFG_ODR: {:08b}\n", cfgodr);

    println!("current config: {:?}", msa301.get_config().unwrap());                 
    
    thread::sleep(Duration::from_millis(500));

    //msa301.set_power_mode(PowerMode::LowPower);
    
    let pwrbw = msa301.read_register(Registers::PWR_BW).unwrap();                                
    println!("PWR_BW: {:08b}\n", pwrbw);
                    

    println!("current config: {:?}", msa301.get_config().unwrap());

    thread::sleep(Duration::from_millis(500));

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z);     

    thread::sleep(Duration::from_millis(500));

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z);     

    println!("Scale factor: {}", msa301.get_scale().unwrap());

    println!("change range to 2g");

    msa301.set_range(Range::_2g).unwrap();

    let resrange = msa301.read_register(Registers::RES_RANGE).unwrap();

    println!("Scale factor: {}", msa301.get_scale().unwrap());

    println!("current config: {:?}", msa301.get_config().unwrap());

    thread::sleep(Duration::from_millis(500));

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z); 

    thread::sleep(Duration::from_millis(500));

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z); 


    println!("Scale factor: {}", msa301.get_scale().unwrap());


    thread::sleep(Duration::from_millis(500));

    let (x,y,z) = msa301.read_accel().unwrap();
            
    println!("x: {}, y: {}, z: {}\r\n", x,y, z); 

    /*

    loop {
        }
    */
}
