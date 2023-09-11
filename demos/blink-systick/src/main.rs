#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as rt;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_hal as hal;
use hal::oscillator::Clocks;
use rt::entry;
use rt::exception;

#[entry]
fn main() -> ! {
    let clocks = Clocks::init();
    clocks.enable_gpio();
    loop {
        cortex_m::asm::wfe();
    }
}

#[exception]
fn SysTick() {}
