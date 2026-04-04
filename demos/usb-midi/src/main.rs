//! USB MIDI device demo for the SLSTK3400A.
//!
//! Plays "Twinkle Twinkle Little Star" in a loop once configured.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_pac as pac;
use efm32hg322_usb::midi::{self, MidiClass, MidiHandler};
use efm32hg322_usb::UsbDevice;

struct TwinkleTune;

impl MidiHandler for TwinkleTune {
    fn tune(&self) -> &[(u8, u32)] {
        &TUNE
    }
}

// "Twinkle Twinkle Little Star" - (MIDI note, duration in ms)
#[rustfmt::skip]
static TUNE: [(u8, u32); 42] = [
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
];

efm32hg322_usb::usb_device!(MidiClass);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let dev = UsbDevice::init(&p.cmu, &p.usb, MidiClass, midi::usb_config());

    defmt::info!("USB MIDI device ready - connect cable now");
    usb_start(dev);
    midi::run(&TwinkleTune, |pkt| usb_with_bus(|bus| bus.ep_write(1, pkt)));
}
