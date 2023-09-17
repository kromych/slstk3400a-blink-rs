//! The Core-M SysTick peripheral
//!
//! This is implemented in efm32gg-hal and not a generic Cortex-M HAL because the relevant
//! functionality depends not only on the peripheral but also on the (EFM32) clock source that
//! feeds it.
//!
//! This looks quite a bit like the stm32f30x-hal delays.rs
//! <https://github.com/japaric/stm32f30x-hal/blob/master/src/delay.rs> -- I tried out of curiosity
//! to come up with an own solution, and it turns out thsi is how it needs to be done
//!
//! FIXME: factor out the common parts (which should be everything except the actual numbers for
//! the clock frequency depending on the SystClkSource) into ... core-m-hal?

use crate::time_util::{self as time, Hertz};
use cortex_m;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub trait SystickExt {
    fn constrain(self, freq: Hertz) -> Systick;
}

impl SystickExt for cortex_m::peripheral::SYST {
    fn constrain(self, freq: Hertz) -> Systick {
        Systick {
            registerblock: self,
            freq,
        }
    }
}

pub struct Systick {
    // We could use a zero-sized abstraction here like we do for GPIO pins, but it's internal
    // anyway and I don't care about those 4 byte right now; feel free to bend it.
    registerblock: cortex_m::peripheral::SYST,
    freq: time::Hertz,
}

impl<UXX> DelayUs<UXX> for Systick
where
    UXX: Into<u32>,
{
    fn delay_us(&mut self, us: UXX) {
        // FIXME this assumes clock rate is in Hz, which usually holds.
        // Instead of dividing by 1_000_000, divide by 1000 two times
        // to avoid getting 0 if `freq` is less than a MHz.
        let factor = self.freq.0 / 1000;
        // Just trigger the assertion...
        let ticks = factor.checked_mul(us.into() / 1000).unwrap_or(1 << 24);

        // FIXME: If we can show that all the above calculation can be done in LTO, then I'd be
        // much more comfortable adding logic that goes into loops for sleeps exceeding one systick
        // wrap (which is about 2s on typical 14MHz devices).
        assert!(ticks < (1 << 24));

        self.registerblock.set_reload(ticks);
        self.registerblock.clear_current();
        self.registerblock.enable_counter();

        while !self.registerblock.has_wrapped() {}
        self.registerblock.disable_counter();
    }
}

// Limited to u16 because waiting for 2**16 or more ms already exceeds what the DelayUs
// implementation can do even on a 1MHz clock, and lower clock frequencies can't be expressed
// anyway in that implementation.
impl<UXX> DelayMs<UXX> for Systick
where
    UXX: Into<u16>,
{
    fn delay_ms(&mut self, ms: UXX) {
        let ms: u16 = ms.into();
        let ms: u32 = ms.into();
        self.delay_us(ms * 1000);
    }
}
