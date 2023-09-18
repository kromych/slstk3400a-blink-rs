#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use embedded_hal::prelude::*;
use embedded_hal::watchdog::WatchdogDisable;
use hal::gpio::GPIOExt;
use hal::oscillator::hfrco::DEFAULT_HFRCO_FREQUENCY;
use hal::systick::SystickExt;
use hal::watchdog::WatchdogExt;
use slstk3400a::SlStk3400a;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    // Enable GPIO clock to enable GPIO as outputs.
    p.CMU.hfperclken0.write(|w| w.gpio().set_bit());

    let gpio = p.GPIO.constrain().split();
    let mut board = SlStk3400a::new(gpio).unwrap();
    let leds = board.leds_mut();

    // The HFRCO oscillator is a low energy oscillator with extremely short wake-up time. Therefore,
    // this oscillator is always chosen by hardware as the clock source for HFCLK when the device starts up (e.g.
    // after reset and after waking up from EM2 and EM3). After reset, the HFRCO frequency is 14 MHz.
    let mut systick = cp.SYST;
    systick.set_clock_source(SystClkSource::Core);
    let mut systick = systick.constrain(DEFAULT_HFRCO_FREQUENCY);

    let mut count = 0usize;
    loop {
        leds[count & 1].on();
        systick.delay_ms(1000u16);

        defmt::info!("Hello, world #{}!", count);

        leds[count & 1].off();
        systick.delay_ms(1000u16);

        count = count.wrapping_add(1);
    }
}
