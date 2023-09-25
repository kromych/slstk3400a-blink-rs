use efm32hg322_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::gpio::{
    pins::{PF4, PF5},
    Normal, Output, PushPull,
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

pub type LED0 = LED<PF4<Output<PushPull<Normal>>>>;
pub type LED1 = LED<PF5<Output<PushPull<Normal>>>>;

pub struct LEDs {
    pub led0: LED0,
    pub led1: LED1,
}

impl LEDs {
    pub fn new(led0: PF4<Output<PushPull<Normal>>>, led1: PF5<Output<PushPull<Normal>>>) -> Self {
        LEDs {
            led0: LED(led0),
            led1: LED(led1),
        }
    }
}

impl<T: OutputPin> LedTrait for LED<T> {
    fn on(&mut self) {
        let _ = self.0.set_high();
    }

    fn off(&mut self) {
        let _ = self.0.set_low();
    }
}
