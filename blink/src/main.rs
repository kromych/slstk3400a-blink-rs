#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use embedded_hal::prelude::*;
use embedded_hal::watchdog::WatchdogDisable;
use slstk3400a::leds::LedTrait;
use slstk3400a::SlStk3400a;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = SlStk3400a::new().unwrap();
    let leds: [&mut dyn LedTrait; 2] = [&mut board.leds.led0, &mut board.leds.led1];

    // If the Watchdog is not reset/disabled, the board will reboot.
    board.watchdog.disable();

    let mut count = 0usize;
    loop {
        leds[count & 1].on();
        board.delay.delay_ms(1000u16);

        defmt::info!("Hello, world #{}!", count);

        leds[count & 1].off();
        board.delay.delay_ms(1000u16);

        count = count.wrapping_add(1);
    }
}
