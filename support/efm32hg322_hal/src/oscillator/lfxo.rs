use super::Oscillator;
use crate::registers;
use crate::time_util::Hertz;

pub const LFXO_FREQUENCY: Hertz = Hertz(32_768);

pub struct LFXO {
    _private: (),
}

impl LFXO {
    pub fn init() -> Self {
        Self { _private: () }
    }
}

impl Oscillator for LFXO {
    fn freq(&self) -> Hertz {
        LFXO_FREQUENCY
    }

    fn enable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.lfxoen().set_bit());
        loop {
            if cmu.status.read().lfxordy().bit() {
                break;
            }
        }
    }

    fn disable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.lfxodis().set_bit());
        loop {
            if cmu.status.read().lfxordy().bit() {
                break;
            }
        }
    }

    fn select_hfclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.hfclksel().lfxo());
        loop {
            if cmu.status.read().lfxosel().bit() {
                break;
            }
        }
    }

    fn select_usbclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.usbcclksel().lfxo());
        loop {
            if cmu.status.read().lfxosel().bit() {
                break;
            }
        }
    }
}
