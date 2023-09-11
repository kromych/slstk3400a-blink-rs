use super::Oscillator;
use crate::registers;
use crate::time_util::Hertz;

pub struct HFXO {
    _private: (),
}

impl HFXO {
    pub fn init() -> Self {
        Self { _private: () }
    }
}

impl Oscillator for HFXO {
    fn freq(&self) -> Hertz {
        Hertz(24_000_000)
    }

    fn enable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.hfxoen().set_bit());
        loop {
            if cmu.status.read().hfxordy().bit() {
                break;
            }
        }
    }

    fn disable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.hfxodis().set_bit());
        loop {
            if cmu.status.read().hfxordy().bit() {
                break;
            }
        }
    }

    fn select_hfclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.hfclksel().hfxo());
        loop {
            if cmu.status.read().hfxosel().bit() {
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
