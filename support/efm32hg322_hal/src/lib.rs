#![no_std]

pub mod cmu;
pub mod delay;
pub mod gpio;
pub mod rtc;
pub mod systick;
pub mod time_util;
pub mod timer;
pub mod watchdog;

pub use efm32hg322_pac as registers;
