//! USB HID keyboard demo for the SLSTK3400A.
//!
//! Types "Hello from EFM32!\n" every 5 seconds.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_pac as pac;
use efm32hg322_usb::hid_keyboard::{self, HidKeyboardClass, HidKeyboardHandler};
use efm32hg322_usb::UsbDevice;

struct Greeting;

impl HidKeyboardHandler for Greeting {
    fn message(&self) -> &[u8] {
        b"Hello from EFM32!\n"
    }
}

efm32hg322_usb::usb_device!(HidKeyboardClass);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        HidKeyboardClass,
        hid_keyboard::usb_config(),
    );

    defmt::info!("USB HID keyboard ready - connect cable now");
    usb_start(dev);
    hid_keyboard::run(&Greeting, |report| {
        usb_with_bus(|bus| bus.ep_write(1, report))
    });
}
