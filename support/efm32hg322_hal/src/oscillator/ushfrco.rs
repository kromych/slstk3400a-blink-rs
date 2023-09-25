use super::FreqBand;
use super::Oscillator;
use crate::registers;
use crate::time_util::Hertz;
use crate::OscillatorFreqSet;

pub const USHFRCO_FREQUENCY: Hertz = Hertz(48_000_000);

pub struct USHFRCO {
    _private: (),
}

impl USHFRCO {
    pub fn init() -> Self {
        Self { _private: () }
    }
}

impl Oscillator for USHFRCO {
    fn freq(&self) -> Hertz {
        USHFRCO_FREQUENCY
    }

    fn enable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.ushfrcoen().set_bit());
        loop {
            if cmu.status.read().ushfrcordy().bit() {
                break;
            }
        }
    }

    fn disable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.ushfrcodis().set_bit());
        loop {
            if cmu.status.read().ushfrcordy().bit() {
                break;
            }
        }
    }

    fn select_hfclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.hfclksel().ushfrcodiv2());
        loop {
            if cmu.status.read().ushfrcodiv2sel().bit() {
                break;
            }
        }
    }

    fn select_usbclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.usbcclksel().ushfrco());
        // cmu.usbcrctrl;
        // cmu.ushfrcoctrl;
        // cmu.ushfrcotune;
        // cmu.ushfrcoconf;
        loop {
            if cmu.status.read().usbclfrcosel().bit() {
                break;
            }
        }
    }
}

impl OscillatorFreqSet for USHFRCO {
    fn set_freq(&mut self, band: super::FreqBand) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        // cmu.usbcrctrl;
        // cmu.ushfrcoctrl;
        // cmu.ushfrcotune;
        // cmu.ushfrcoconf;

        match band {
            FreqBand::Freq24MHz => cmu.ushfrcoconf.write(|w| w.band()._24mhz()),
            FreqBand::Freq48MHz => cmu.ushfrcoconf.write(|w| w.band()._48mhz()),
            _ => unimplemented!(),
        }
    }
}
