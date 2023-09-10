#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as rt;
use defmt_rtt as _;
use panic_halt as _;

use embedded_hal::watchdog::WatchdogDisable;
use rt::entry;
use rt::exception;
use slstk3400a::SlStk3400a;

#[entry]
fn main() -> ! {
    let mut board = SlStk3400a::new().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    board.watchdog.disable();

    loop {}
}

#[exception]
fn SysTick() {}
