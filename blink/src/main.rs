#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use embedded_hal::prelude::*;
use embedded_hal::watchdog::WatchdogDisable;
use slstk3400a::SlStk3400a;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = SlStk3400a::new().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    board.watchdog.disable();

    let mut count = 0u32;
    while count < 1000 {
        defmt::info!("Hello, world #{}!", count);

        board.delay.delay_ms(500u16);
        count = count.wrapping_add(1);
    }

    loop {
        cortex_m::asm::wfe();
    }
}
