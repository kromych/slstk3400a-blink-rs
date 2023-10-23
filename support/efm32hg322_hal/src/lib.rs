#![no_std]

pub use pac::cmu::hfcoreclkdiv::HFCORECLKDIV_A as HfCoreClkDiv;
pub use pac::cmu::hfperclkdiv::HFPERCLKDIV_A as HfPerClkDiv;
pub use pac::cmu::hfrcoctrl::BAND_A as HfrcoBand;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HfClkDiv {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
    Div16 = 4,
    Div32 = 5,
    Div64 = 6,
    Div128 = 7,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HfCoreClkLeDiv {
    Div2 = 0,
    Div4 = 1,
}

pub mod clocks;
pub mod gpio;
pub mod rtc;
pub mod systick;
pub mod time_util;
pub mod timer;
pub mod watchdog;

pub use efm32hg322_pac as pac;
