//! print data received from MSA301 sensor over I2C

#![no_std]
#![no_main]

use adafruit_feather_rp2040::hal;

use adafruit_feather_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
    
};

// USB device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communication Class Device support
use usbd_serial::SerialPort;

use embedded_hal::digital::v2::OutputPin;

use cortex_m_rt::entry;
use embedded_time::rate::*;
//use embedded_time::duration::FixedPoint;
use panic_halt as _;

use msa301::*;

use core::fmt;

use arrayvec::ArrayString;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
                        XOSC_CRYSTAL_FREQ, 
                        pac.XOSC, 
                        pac.CLOCKS, 
                        pac.PLL_SYS,
                        pac.PLL_USB,                        
                        &mut pac.RESETS, 
                        &mut watchdog,
                    )
                    .ok().unwrap();

    // Set up USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
                pac.USBCTRL_REGS,
                pac.USBCTRL_DPRAM,
                clocks.usb_clock,
                true,
                &mut pac.RESETS,
    ));

    // Set up USB Communication Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(2) // from https://www.usb.org/defined-class-codes
                .build();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
                pac.IO_BANK0,
                pac.PADS_BANK0,
                sio.gpio_bank0,
                &mut pac.RESETS,
            );
    

    // configure I2C pins
    let sda_pin = pins.sda.into_mode();
    let scl_pin = pins.scl.into_mode();

    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let mut msa301 = MSA301::new(i2c);

    loop {
        
        if ! usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        
        let mut text_buf = ArrayString::<[u8;64]>::new();                

        let id = msa301.get_device_id().unwrap();

        fmt_output(&mut text_buf, id);

        serial.write(text_buf.as_bytes());

        //let _ = serial.write(b"Hello world!\r\n");

    }


}

pub fn fmt_output(buf: &mut ArrayString<[u8;64]>, val: u8) {
    fmt::write(buf, format_args!("My name is {}\r\n", val)).unwrap();
}