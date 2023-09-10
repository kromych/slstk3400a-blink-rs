//! CMU (Clock Management Unit)
//!
//! Introduce new clock abstraction:
//! - `Clocks` which can be configured further, e.g. change current oscillator, frequency.
//! - `FrozenClocks` which freeze the clock configuration and split further into peripheral clock
//! enablers

pub mod clocks;
pub mod oscillator;

use crate::{registers, time_util as time};
use oscillator::{Oscillator, HFRCO};

pub trait CMUExt {
    fn constrain(self) -> Clocks<HFRCO>;
}

impl CMUExt for registers::CMU {
    fn constrain(self) -> Clocks<HFRCO> {
        Clocks::<HFRCO>::init(self)
    }
}

pub struct Clocks<O: Oscillator> {
    pub cmu: registers::CMU,
    osc: O,
}

impl<O: Oscillator> Clocks<O> {
    pub fn init(cmu: registers::CMU) -> Clocks<HFRCO> {
        let clocks = Clocks {
            osc: oscillator::default(),
            cmu,
        };

        clocks.osc.select();

        clocks
    }

    pub fn freeze(self) -> FrozenClocks {
        #[cfg(feature = "use_le_peripherals")]
        clocks::reset_le_clocks(&self);

        FrozenClocks {
            cpufreq: self.freq(),
            adc0: clocks::ADC0::constrain(),
            gpio: clocks::GPIO::constrain(),
            timer0: clocks::TIMER0::constrain(),
            timer1: clocks::TIMER1::constrain(),

            #[cfg(feature = "_has_timer2")]
            timer2: clocks::TIMER2::constrain(),

            i2c0: clocks::I2C0::constrain(),
            idac0: clocks::IDAC0::constrain(),
            acmp0: clocks::ACMP0::constrain(),
            prs: clocks::PRS::constrain(),
            usart0: clocks::USART0::constrain(),
            usart1: clocks::USART1::constrain(),
            vcmp: clocks::VCMP::constrain(),

            #[cfg(feature = "use_le_peripherals")]
            rtc: clocks::RTC::constrain(),

            #[cfg(feature = "use_le_peripherals")]
            leuart0: clocks::LEUART0::constrain(),

            #[cfg(feature = "use_le_peripherals")]
            usble: clocks::USBLE::constrain(),
        }
    }

    pub fn set_freq(&mut self, f: O::FreqBand) {
        self.osc.set_freq(f);
    }

    pub fn freq(&self) -> time::MegaHertz {
        self.osc.freq().into()
    }

    pub fn change_oscillator<T: Oscillator>(self, osc: T) -> Clocks<T> {
        Clocks { osc, cmu: self.cmu }
    }
}

pub struct FrozenClocks {
    cpufreq: time::MegaHertz,
    pub adc0: clocks::ADC0,
    pub gpio: clocks::GPIO,
    pub timer0: clocks::TIMER0,
    pub timer1: clocks::TIMER1,

    #[cfg(feature = "_has_timer2")]
    pub timer2: clocks::TIMER2,

    pub i2c0: clocks::I2C0,
    pub idac0: clocks::IDAC0,
    pub acmp0: clocks::ACMP0,
    pub prs: clocks::PRS,
    pub usart0: clocks::USART0,
    pub usart1: clocks::USART1,
    pub vcmp: clocks::VCMP,

    #[cfg(feature = "use_le_peripherals")]
    pub rtc: clocks::RTC,

    #[cfg(feature = "use_le_peripherals")]
    pub leuart0: clocks::LEUART0,

    #[cfg(feature = "use_le_peripherals")]
    pub usble: clocks::USBLE,
}

impl FrozenClocks {
    pub fn freq(&self) -> time::MegaHertz {
        self.cpufreq
    }
}
