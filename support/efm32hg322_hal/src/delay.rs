use crate::systick;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub enum CountProvider {
    SysTick(systick::SystickDelay),
}

pub struct Delay {
    provider: CountProvider,
}

impl Delay {
    pub fn new(p: CountProvider) -> Self {
        Self { provider: p }
    }
}

impl<UXX> DelayMs<UXX> for Delay
where
    UXX: Into<u16>,
{
    fn delay_ms(&mut self, ms: UXX) {
        match self.provider {
            CountProvider::SysTick(ref mut syst) => syst.delay_ms(ms),
        }
    }
}

impl<UXX> DelayUs<UXX> for Delay
where
    UXX: Into<u32>,
{
    fn delay_us(&mut self, us: UXX) {
        match self.provider {
            CountProvider::SysTick(ref mut syst) => syst.delay_us(us),
        }
    }
}
