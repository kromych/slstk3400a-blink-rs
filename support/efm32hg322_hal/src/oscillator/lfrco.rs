use super::Oscillator;
use crate::registers;
use crate::time_util::Hertz;

pub struct LFRCO {
    _private: (),
}

impl LFRCO {
    pub fn init() -> Self {
        Self { _private: () }
    }
}

impl Oscillator for LFRCO {
    fn freq(&self) -> Hertz {
        Hertz(32768)
    }

    fn enable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.lfrcoen().set_bit());
        loop {
            if cmu.status.read().lfrcordy().bit() {
                break;
            }
        }
    }

    fn disable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.lfrcodis().set_bit());
        loop {
            if cmu.status.read().lfrcordy().bit() {
                break;
            }
        }
    }

    fn select_hfclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.hfclksel().lfrco());
        loop {
            if cmu.status.read().lfrcosel().bit() {
                break;
            }
        }
    }

    fn select_usbclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.usbcclksel().lfrco());
        loop {
            if cmu.status.read().lfrcosel().bit() {
                break;
            }
        }
    }
}
