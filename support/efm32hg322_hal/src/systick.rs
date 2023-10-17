//! The Core-M SysTick peripheral

use crate::time_util::{self as time, Hertz};
use cortex_m;

const SYSTICK_MAX_COUNT: u32 = 1 << 24;

pub trait SystickExt {
    fn constrain(self, freq: Hertz) -> Systick;
}

impl SystickExt for cortex_m::peripheral::SYST {
    fn constrain(self, osc_freq: Hertz) -> Systick {
        Systick {
            registerblock: self,
            osc_freq,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DelayError {
    TooLong,
}

pub struct Systick {
    // We could use a zero-sized abstraction here like we do for GPIO pins, but it's internal
    // anyway and I don't care about those 4 byte right now; feel free to bend it.
    registerblock: cortex_m::peripheral::SYST,
    osc_freq: time::Hertz,
}

impl Systick {
    pub fn enable_interrupt(&mut self, int_freq: Hertz) {
        self.disable_interrupt();
        self.registerblock
            .set_reload(self.osc_freq.0 / int_freq.0 - 1);
        self.registerblock.clear_current();
        self.registerblock.enable_interrupt();
        self.registerblock.enable_counter();
    }

    pub fn disable_interrupt(&mut self) {
        self.registerblock.disable_counter();
        self.registerblock.disable_interrupt();
    }

    pub fn max_delay_us(&self) -> u32 {
        ((SYSTICK_MAX_COUNT as u64) * 1_000_000 / (self.osc_freq.0 as u64)) as u32
    }

    pub fn delay_us(&mut self, us: u32) -> Result<(), DelayError> {
        if us == 0 {
            return Ok(());
        }

        let factor = (us as u64)
            .checked_mul(self.osc_freq.0 as u64)
            .ok_or(DelayError::TooLong)?;

        let ticks = (factor / 1_000_000) as u32;
        if ticks >= SYSTICK_MAX_COUNT {
            return Err(DelayError::TooLong);
        }

        self.registerblock.set_reload(ticks - 1);
        self.registerblock.clear_current();
        self.registerblock.enable_counter();

        while !self.registerblock.has_wrapped() {}
        self.registerblock.disable_counter();

        Ok(())
    }
}
