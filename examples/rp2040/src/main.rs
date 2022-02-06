//! simple blink using just the RP2040-HAL without the BSP crate

#![no_std]
#![no_main]

use cortex_m_rt::entry;

use panic_halt as _;

use rp2040_hal as hal;

use hal::pac;

use embedded_hal::digital::v2::OutputPin;
use embedded_time::rate::Extensions;
use embedded_time::fixed_point::FixedPoint;
use hal::clocks::Clock;


// USB device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communication Class Device support
use usbd_serial::SerialPort;

// MSA301 driver
use msa301::*;
use msa301::register::Registers;

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
    let core = pac::CorePeripherals::take().unwrap();

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

    

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    
    let mut msa301 = MSA301::new(i2c);
    
    loop {
    
        if ! usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
    
        let mut text_buf = ArrayString::<[u8;64]>::new();

        //let id = msa301.get_device_id().unwrap();

        // let lo = msa301.read_register(msa301::register::Registers::ZAXIS_L).unwrap();
        let reg = msa301.read_register(msa301::register::Registers::CFG_ODR).unwrap();

        fmt_output(&mut text_buf, reg);

        serial.write(text_buf.as_bytes());
    }

}

// helper function to print numeric data
pub fn fmt_output(buf: &mut ArrayString<[u8;64]>, a: u8) {
    fmt::write(buf, format_args!("register value: {}\r\n", a)).unwrap();
}