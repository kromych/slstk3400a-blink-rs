#![no_std]

use efm32hg322_hal as hal;
use hal::gpio::Pins;
use leds::LEDs;

pub mod leds;

pub struct SlStk3400a {
    pub leds: LEDs,
}

impl SlStk3400a {
    pub fn new(gpio: Pins) -> Option<Self> {
        Some(Self {
            leds: LEDs::new(gpio.pf4.into(), gpio.pf5.into()),
        })
    }
}
