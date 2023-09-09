#![no_main]
#![no_std]

// Panic handler.
use panic_halt as _;
// Vector table, exceptions, and the interrupt table for Cortex M.
use cortex_m as _;
use cortex_m_rt as _;
// Logging via RTT
use defmt_rtt as _;
// Peripheral access crate, default interrupt handlers.
use efm32hg_pac::efm32hg322 as pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    // If the Watchdog is not reset, the board will reboot.
    peripherals.WDOG.ctrl.reset();

    defmt::info!("Hello, world!");

    loop {
        cortex_m::asm::wfe();
    }
}
