use efm32hg322_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::gpio::{
    pins::{PF4, PF5},
    Normal, OpenDrain, Output, PullUp,
};

pub struct LED<T>(T)
where
    T: ?Sized;

pub trait LedTrait {
    /// Turn on the led.
    fn on(&mut self);

    /// Turn off the led.
    fn off(&mut self);
}

pub struct LEDs {
    pub led0: LED<PF4<Output<OpenDrain<Normal, PullUp>>>>,
    pub led1: LED<PF5<Output<OpenDrain<Normal, PullUp>>>>,
}

impl LEDs {
    pub fn new(
        led0: PF4<Output<OpenDrain<Normal, PullUp>>>,
        led1: PF5<Output<OpenDrain<Normal, PullUp>>>,
    ) -> Self {
        LEDs {
            led0: LED(led0),
            led1: LED(led1),
        }
    }
}

impl<T: OutputPin> LedTrait for LED<T> {
    fn on(&mut self) {
        let _ = self.0.set_low();
    }

    fn off(&mut self) {
        let _ = self.0.set_high();
    }
}
