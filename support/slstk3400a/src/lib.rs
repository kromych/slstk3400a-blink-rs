#![no_std]

use efm32hg322_hal as hal;
use hal::gpio::Pins;
use leds::LEDs;
use leds::LedTrait;
use push_buttons::PushButtonTrait;
use push_buttons::PushButtons;

pub mod leds;
pub mod push_buttons;

pub struct SlStk3400a {
    pub leds: LEDs,
    pub btns: PushButtons,
}

impl SlStk3400a {
    pub fn new(gpio: Pins) -> Option<Self> {
        Some(Self {
            leds: LEDs::new(gpio.pf4.into(), gpio.pf5.into()),
            btns: PushButtons::new(gpio.pc9.into(), gpio.pc10.into()),
        })
    }

    pub fn leds_mut(&mut self) -> [&mut dyn LedTrait; 2] {
        [&mut self.leds.led0, &mut self.leds.led1]
    }

    pub fn btns_mut(&mut self) -> [&mut dyn PushButtonTrait; 2] {
        [&mut self.btns.btn0, &mut self.btns.btn1]
    }
}
