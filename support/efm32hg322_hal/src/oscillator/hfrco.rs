use super::FreqBand;
use super::Oscillator;
use super::OscillatorFreqSet;
use crate::registers;
use crate::time_util::Hertz;

pub const DEFAULT_HFRCO_FREQUENCY: Hertz = Hertz(14_000_000);

pub struct HFRCO {
    _private: (),
}

impl HFRCO {
    pub fn init() -> Self {
        Self { _private: () }
    }
}

impl Oscillator for HFRCO {
    fn freq(&self) -> Hertz {
        let cmu = unsafe { &*registers::CMU::ptr() };
        let band = cmu.hfrcoctrl.read().band().bits();
        match band {
            0 => Hertz(1_000_000),
            1 => Hertz(7_000_000),
            2 => Hertz(11_000_000),
            3 => Hertz(14_000_000),
            4 => Hertz(21_000_000),
            _ => unreachable!("no arbitrary freq. allowed"),
        }
    }

    fn enable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.hfrcoen().set_bit());
        loop {
            if cmu.status.read().hfrcordy().bit() {
                break;
            }
        }
    }

    fn disable(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.oscencmd.write(|w| w.hfrcodis().set_bit());
        loop {
            if cmu.status.read().hfrcordy().bit() {
                break;
            }
        }
    }

    fn select_hfclk(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.hfclksel().hfrco());
        loop {
            if cmu.status.read().hfrcosel().bit() {
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

impl OscillatorFreqSet for HFRCO {
    fn set_freq(&mut self, band: FreqBand) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        let devinfo = unsafe { &*registers::DEVINFO::ptr() };
        cmu.hfrcoctrl.write(|w| unsafe {
            match band {
                FreqBand::Freq1MHz => w
                    .band()
                    ._1mhz()
                    .tuning()
                    .bits(devinfo.hfrcocal0.read().band1().bits()),
                FreqBand::Freq7MHz => w
                    .band()
                    ._7mhz()
                    .tuning()
                    .bits(devinfo.hfrcocal0.read().band7().bits()),
                FreqBand::Freq11MHz => w
                    .band()
                    ._11mhz()
                    .tuning()
                    .bits(devinfo.hfrcocal0.read().band11().bits()),
                FreqBand::Freq14MHz => w
                    .band()
                    ._14mhz()
                    .tuning()
                    .bits(devinfo.hfrcocal0.read().band14().bits()),
                FreqBand::Freq21MHz => w
                    .band()
                    ._21mhz()
                    .tuning()
                    .bits(devinfo.hfrcocal1.read().band21().bits()),
                _ => unimplemented!(),
            }
        });
    }
}
