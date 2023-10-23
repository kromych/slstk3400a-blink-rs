//! This demo shows how to blink an LED and use the SysTick delay.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use embedded_hal::watchdog::WatchdogDisable;
use hal::clocks::enable_gpio_clock;
use hal::clocks::ClockSetup;
use hal::clocks::ClockSource;
use hal::gpio::GPIOExt;
use hal::systick::SystickExt;
use hal::watchdog::WatchdogExt;
use hal::HfClkDiv;
use hal::HfrcoBand;
use slstk3400a::SlStk3400a;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    // Enable GPIO clock to enable GPIO as outputs.
    enable_gpio_clock();

    let gpio = p.GPIO.constrain().split();
    let mut board = SlStk3400a::new(gpio).unwrap();
    let leds = board.leds_mut();

    // The HFRCO oscillator is a low energy oscillator with extremely short wake-up time. Therefore,
    // this oscillator is always chosen by hardware as the clock source for HFCLK when the device starts up (e.g.
    // after reset and after waking up from EM2 and EM3). After reset, the HFRCO frequency is 14 MHz.

    let cf = hal::clocks::get_clock_config().expect("Must be able to get clock config");
    defmt::info!("Clock configuration after reset: {}", cf);
    defmt::flush();

    let cs = ClockSetup {
        source: ClockSource::HFRCO(HfrcoBand::_7MHZ),
        hfclkdiv: HfClkDiv::Div2,
        ..Default::default()
    };
    let cf = hal::clocks::set_clock_config(&cs).expect("Clock configuration must succeed");
    defmt::info!("Clock configuration: {}", cf);
    defmt::flush();

    let mut systick = cp.SYST;
    systick.set_clock_source(SystClkSource::Core);
    let mut systick = systick.constrain(&cf);

    let max_delay_us = systick.max_delay_us();
    defmt::info!("Max delay: {} us", max_delay_us);

    defmt::info!("Starting blinking");

    let delay_us = core::cmp::min(1_000_000u32, max_delay_us);
    let mut count = 0usize;
    loop {
        leds[count & 1].on();
        leds[(count + 1) & 1].off();
        systick.delay_us(delay_us).expect("Delay must succeed");

        defmt::info!("Hello, world #{}!", count);

        count = count.wrapping_add(1);
    }
}
