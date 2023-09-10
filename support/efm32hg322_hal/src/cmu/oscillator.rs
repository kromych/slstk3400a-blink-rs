use crate::time_util as time;

pub mod hfrco;
pub use hfrco::HFRCO;

pub trait Oscillator {
    type FreqBand: Into<time::MegaHertz>;
    fn freq(&self) -> Self::FreqBand;
    fn set_freq(&mut self, band: Self::FreqBand);
    fn select(&self);
}

pub(crate) fn default() -> HFRCO {
    HFRCO::init()
}
