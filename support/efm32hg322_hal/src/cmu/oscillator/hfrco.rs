use super::Oscillator;
use crate::registers;
use crate::time_util as time;

#[repr(u8)]
pub enum Band {
    Freq1MHz = 0,
    Freq7MHz = 1,
    Freq11MHz = 2,
    Freq14MHz = 3,
    Freq21MHz = 4,
}

pub struct HFRCO {
    _private: (),
}

impl From<Band> for time::MegaHertz {
    fn from(fb: Band) -> time::MegaHertz {
        time::MegaHertz(fb as _)
    }
}

impl HFRCO {
    /// Contract: this supposed to be called only once within parent module
    pub(super) fn init() -> Self {
        let mut osc = Self { _private: () };
        osc.set_freq(Band::Freq14MHz);
        osc.enable();
        osc
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
}

impl Oscillator for HFRCO {
    type FreqBand = Band;

    fn freq(&self) -> Self::FreqBand {
        let cmu = unsafe { &*registers::CMU::ptr() };
        let band = cmu.hfrcoctrl.read().band().bits();
        match band {
            0 => Band::Freq1MHz,
            1 => Band::Freq7MHz,
            2 => Band::Freq11MHz,
            3 => Band::Freq14MHz,
            4 => Band::Freq21MHz,
            _ => unreachable!("no arbitrary freq. allowed"),
        }
    }

    #[rustfmt::skip]
    fn set_freq(&mut self, band: Self::FreqBand) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        let devinfo = unsafe { &*registers::DEVINFO::ptr() };
        cmu.hfrcoctrl.write(|w| unsafe {
            match band {
                Band::Freq1MHz => w
                    .band()._1mhz()
                    .tuning().bits(
                        devinfo.hfrcocal0.read().band1().bits()
                    ),
                Band::Freq7MHz => w
                    .band()._7mhz()
                    .tuning().bits(
                        devinfo.hfrcocal0.read().band7().bits()
                    ),
                Band::Freq11MHz => w
                    .band()._11mhz()
                    .tuning().bits(
                        devinfo.hfrcocal0.read().band11().bits()
                    ),
                Band::Freq14MHz => w
                    .band()._14mhz()
                    .tuning().bits(
                        devinfo.hfrcocal0.read().band14().bits()
                    ),
                Band::Freq21MHz => w
                    .band()._21mhz()
                    .tuning().bits(
                        devinfo.hfrcocal1.read().band21().bits()
                    ),
            }
        });
    }

    fn select(&self) {
        let cmu = unsafe { &*registers::CMU::ptr() };
        cmu.cmd.write(|w| w.hfclksel().hfrco());
        loop {
            if cmu.status.read().hfrcosel().bit() {
                break;
            }
        }
    }
}
