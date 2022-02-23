//! Test of MSA301 sensor with RP2040 board

#![no_std]
#![no_main]

use cortex_m_rt::entry;


use panic_halt as _;

use rp2040_hal as hal;

use hal::pac;

//use embedded_hal::digital::v2::OutputPin;
use embedded_time::rate::Extensions;


// USB device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communication Class Device support
use usbd_serial::SerialPort;

// MSA301 driver
use msa301::*;
use msa301::config::AccelConfig;

// necessary for printing some text
use core::fmt;
use arrayvec::ArrayString;  

// The linker will place this boot block at the start of the program image.
// This is needed to help the ROM bootloader get the code up and running.
#[link_section = ".boot2"]
#[used]
// Adafruit Feather uses this boot loader (may be different for different boards)
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS; 

// Adafruit Feather uses this frequency (may be different for different boards)
const XTAL_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // watchdog driver needed by the clock setup
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ, 
        pac.XOSC, 
        pac.CLOCKS, 
        pac.PLL_SYS, 
        pac.PLL_USB, 
        &mut pac.RESETS, 
        &mut watchdog)
        .ok().unwrap();

    // set up USB driver
    let usb_bus = UsbBusAllocator::new(
                hal::usb::UsbBus::new(
                    pac.USBCTRL_REGS, 
                    pac.USBCTRL_DPRAM, 
                    clocks.usb_clock, 
                    true, 
                    &mut pac.RESETS,
                ));

    // set up USB Communication Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // create a USB Device with fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .device_class(2) 
                    .build();

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

    // configure I2C pins
    let sda_pin = pins.gpio2.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio3.into_mode::<hal::gpio::FunctionI2C>();

    // configure I2C bus
    let mut i2c = hal::I2C::i2c1(
        pac.I2C1, 
        sda_pin, 
        scl_pin, 
        400.kHz(), 
        &mut pac.RESETS, 
        clocks.peripheral_clock
    );

    // initialize MSA301 driver with a desired configuration
    let mut msa301 = MSA301::new(i2c, 
                        AccelConfig { 
                            range: Range::_2g,
                            datarate: DataRate::_3_90Hz,
                            bandwidth: BandWidth::_1_95Hz,
                            ..Default::default() }).unwrap();
    
    msa301.pin_config(IntPin::PushPull, IntActive::High).unwrap();
    
    //msa301.new_data_pin(Flag::Enable).unwrap();

    //msa301.new_data_int(Flag::Enable).unwrap();
    
    msa301.set_int_latch(true, IntLatch::NonLatched).unwrap();

    //msa301.single_tap_int(Flag::Enable).unwrap();

    loop {
    
        /*
        if ! usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
    
        //let status = msa301.motion_int_status().unwrap();

        let mut text_buf = ArrayString::<[u8;64]>::new();

        let (x,y,z) = msa301.read_accel().unwrap();
        

        // print the results if there's new data
        //if msa301.is_new_data().unwrap() {

        //tap_output(&mut text_buf, status.single_tap);

        

        
        fmt_output(&mut text_buf, x,y,z);

        serial.write(text_buf.as_bytes());    
 */
     
    }
}

// helper function to print numeric data
pub fn fmt_output(buf: &mut ArrayString<[u8;64]>, x: f32, y: f32, z: f32) {
    fmt::write(buf, format_args!("X: {:.3}, Y: {:.3}, Z: {:.3}\r\n", x,y,z)).unwrap();
}

pub fn tap_output(buf: &mut ArrayString<[u8;64]>, tap: bool) {
    match tap {
        true => fmt::write(buf, format_args!("Tap!\r\n")).unwrap(),
        false => fmt::write(buf, format_args!("No tap :(\r\n")).unwrap(),
    }
    
    
}


