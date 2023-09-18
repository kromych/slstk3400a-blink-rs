//! Oscillator types:
//!
//! 1. LFXO, External, 32.768kHz: Low-Frequency Crystal Oscillator, which operates at lower frequencies, often in the kHz
//!    range. These are typically used for real-time clocks (RTCs) and other low-power operations.
//! 2. HFXO, External, 4..25Mhz: High-Frequency Crystal Oscillator. These can operate in the MHz range and
//!    are often used for the main clock source of a microcontroller or processor.
//! 3. LFRCO, Internal, 32.768kHz: Low-Frequency RC Oscillator, which doesn't use a crystal but rather resistor-capacitor
//!    combinations to generate a clock signal. They are generally less precise than crystal oscillators.
//! 4. HFRCO, Internal, 1..28Mhz: High-Frequency RC Oscillator, the high-frequency counterpart to LFRCO.
//! 5. USHFRCO, Internal, 48/24MHz: Universal Serial HFRCO for USB.
//!
//! There are also:
//!
//! 6. AUXHFRCO, Internal, 1-21Mhz: Auxialiary HRFCO for flash programming.
//! 7. ULFRCO, Internal, 1kHz: Ultra LFRCO for Watchdog.
//!
//! Accuracy, stability, start time, need for calibration and its time, energy consumptions differ for
//! these types. The HFRCO oscillator is a low energy oscillator with extremely short wake-up time. Therefore,
//! this oscillator is always chosen by hardware as the clock source for HFCLK when the device starts up (e.g.
//! after reset and after waking up from EM2 and EM3). After reset, the HFRCO frequency is 14 MHz.
//!
use crate::time_util as time;

pub mod hfrco;
pub mod hfxo;
pub mod lfrco;
pub mod lfxo;
pub mod ushfrco;

use crate::time_util::Hertz;
pub use hfrco::HFRCO;
pub use hfxo::HFXO;
pub use lfrco::LFRCO;
pub use lfxo::LFXO;
pub use ushfrco::USHFRCO;

#[derive(Clone, Copy)]
pub enum FreqBand {
    Freq1MHz,
    Freq7MHz,
    Freq11MHz,
    Freq14MHz,
    Freq21MHz,
    Freq24MHz,
    Freq48MHz,
}

impl From<FreqBand> for Hertz {
    fn from(val: FreqBand) -> Self {
        match val {
            FreqBand::Freq1MHz => Hertz(1_000_000),
            FreqBand::Freq7MHz => Hertz(7_000_000),
            FreqBand::Freq11MHz => Hertz(11_000_000),
            FreqBand::Freq14MHz => Hertz(14_000_000),
            FreqBand::Freq21MHz => Hertz(21_000_000),
            FreqBand::Freq24MHz => Hertz(24_000_000),
            FreqBand::Freq48MHz => Hertz(48_000_000),
        }
    }
}

pub trait Oscillator {
    fn freq(&self) -> time::Hertz;
    fn enable(&self);
    fn disable(&self);
    fn select_hfclk(&self);
    fn select_usbclk(&self);
}

pub trait OscillatorFreqSet {
    fn set_freq(&mut self, band: FreqBand);
}

pub struct Clocks {
    _private: (),
}

impl Clocks {
    pub fn init() -> Self {
        Self { _private: () }
    }

    pub fn set_dividers(&self) {
        let cmu = unsafe { &*crate::registers::CMU::ptr() };
        cmu.hfcoreclkdiv.write(|w| w.hfcoreclkdiv().hfclk16());

        cmu.hfperclkdiv
            .write(|w| w.hfperclken().set_bit().hfperclkdiv().hfclk512());
    }

    pub fn enable_gpio(&self) {
        let cmu = unsafe { &*crate::registers::CMU::ptr() };
        cmu.hfperclken0.write(|w| w.gpio().set_bit());
    }
}
