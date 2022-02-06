//! talking to the MSA301 module over I2C on Raspberry Pi
//! 
//! ID retrieved correctly (decimal 19/ 0x13)

use rppal::i2c::I2c;

use msa301::*;
use msa301::register::Registers;

fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // create a new driver instance with the I2C interface    
    let mut msa301 = MSA301::new(i2c);

    //let id = msa301.get_device_id().unwrap();

    //msa301.enable_axes().unwrap();

    //let reg = msa301.read_register(msa301::register::Registers::Z_COMP).unwrap();

    

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();

    //println!("Device ID: {}\n", id);
    println!("Register value: {}\n", reg);

    println!("Orientation interrupt");

    msa301.orient_int(FLAG::Enable).unwrap();

    println!("Interrupt enabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);

    msa301.orient_int(FLAG::Disable).unwrap();

    println!("Interrupt disabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);


    println!("Single tap interrupt");

    msa301.single_tap_int(FLAG::Enable).unwrap();

    println!("Interrupt enabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);

    msa301.single_tap_int(FLAG::Disable).unwrap();

    println!("Interrupt disabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);


    println!("Double tap interrupt");

    msa301.double_tap_int(FLAG::Enable).unwrap();

    println!("Interrupt enabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);

    msa301.double_tap_int(FLAG::Disable).unwrap();

    println!("Interrupt disabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);



    println!("Active axis X interrupt");

    msa301.active_xaxis_int(FLAG::Enable).unwrap();

    println!("Interrupt enabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);

    msa301.active_xaxis_int(FLAG::Disable).unwrap();

    println!("Interrupt disabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);



    println!("Active axis Y interrupt");

    msa301.active_yaxis_int(FLAG::Enable).unwrap();

    println!("Interrupt enabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);

    msa301.active_yaxis_int(FLAG::Disable).unwrap();

    println!("Interrupt disabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);
    
    
    println!("Active axis Z interrupt");

    msa301.active_zaxis_int(FLAG::Enable).unwrap();

    println!("Interrupt enabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);

    msa301.active_zaxis_int(FLAG::Disable).unwrap();

    println!("Interrupt disabled\n",);

    let reg = msa301.read_register(msa301::register::Registers::INT_SET0).unwrap();
    println!("Register value: {}\n", reg);
    
    /*

    loop {
        /*
        let xlo = msa301.read_register(msa301::register::Registers::XAXIS_L).unwrap();
        let xhi = msa301.read_register(msa301::register::Registers::XAXIS_H).unwrap();

        let ylo = msa301.read_register(msa301::register::Registers::YAXIS_L).unwrap();
        let yhi = msa301.read_register(msa301::register::Registers::YAXIS_H).unwrap();
        */
        let zlo = msa301.read_register(msa301::register::Registers::ZAXIS_L).unwrap();
        let zhi = msa301.read_register(msa301::register::Registers::ZAXIS_H).unwrap();

    //println!("Axis readings: x: {} {}, y: {} {}, z: {} {}\n", xlo, xhi, ylo, yhi, zlo, zhi);
    println!("Axis readings: z: {} {}\n", zlo, zhi);
    }
 */
 
}
