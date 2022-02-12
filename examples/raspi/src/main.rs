//! talking to the MSA301 module over I2C on Raspberry Pi
//! 


use rppal::i2c::I2c;

use msa301::*;
use msa301::config::AccelConfig;
use msa301::register::Registers;

fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // create a new driver instance with the I2C interface and configuration settings      
    let mut msa301 = MSA301::new(i2c,
                            AccelConfig{..Default::default()},
                                /*
                                enable_axes: (true, true, true),
                                powermode: PowerMode::Normal,
                                datarate: DataRate::_125Hz,
                                bandwidth: BandWidth::_62_5Hz,
                                resolution: Res::_14bit,
                                range: Range::_2g,
                                 */
                                ).unwrap();
    
    msa301.begin_accel().unwrap();

    //msa301.set_datarate(DataRate::_125Hz).unwrap();
    //msa301.set_bandwidth(BandWidth::_62_5Hz).unwrap();       
    //msa301.set_power_mode(PowerMode::Normal).unwrap();

    
    let cfgodr = msa301.read_register(Registers::CFG_ODR).unwrap();
    let resrange = msa301.read_register(Registers::RES_RANGE).unwrap();
    let pwrbw = msa301.read_register(Registers::PWR_BW).unwrap();
    
    println!("CFG_ODR: {:08b}\nRES_RANGE: {:08b}\nPWR_BW: {:08b}\n", 
                    cfgodr, resrange, pwrbw);
    
    

    /*
    msa301.set_datarate(DataRate::_125Hz).unwrap();
    msa301.set_bandwidth(BandWidth::_62_5Hz).unwrap();
     */
    let cfgodr = msa301.read_register(Registers::CFG_ODR).unwrap();
    let resrange = msa301.read_register(Registers::RES_RANGE).unwrap();
    let pwrbw = msa301.read_register(Registers::PWR_BW).unwrap();
    
    println!("CFG_ODR: {:08b}\nRES_RANGE: {:08b}\nPWR_BW: {:08b}\n", 
                    cfgodr, resrange, pwrbw);

 
    let xlsb = msa301.read_register(Registers::XAXIS_L).unwrap();
    let xmsb = msa301.read_register(Registers::XAXIS_H).unwrap();
    let ylsb = msa301.read_register(Registers::YAXIS_L).unwrap();
    let ymsb = msa301.read_register(Registers::YAXIS_H).unwrap();
    let zlsb = msa301.read_register(Registers::ZAXIS_L).unwrap();
    let zmsb = msa301.read_register(Registers::ZAXIS_H).unwrap();
    
    let (x,y,z) = msa301.read_accel().unwrap();
    
    println!("x {}, y {}, z {}\r\n", x, y, z);      
    
    
    println!("x LSB {}, x MSB {}\n", xlsb, xmsb);
    println!("y LSB {}, y MSB {}\n", ylsb, ymsb);
    println!("z LSB {}, z MSB {}\n", zlsb, zmsb);
  
    
    loop {
        let (x,y,z) = msa301.read_accel().unwrap();
            
        println!("x: {}, y: {}, z: {}\r\n", x,y, z);        
    }
    
}
