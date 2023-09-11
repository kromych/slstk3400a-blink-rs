#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use embedded_hal::prelude::*;
use embedded_hal::watchdog::WatchdogDisable;
use hal::delay::CountProvider;
use hal::delay::Delay;
use hal::gpio::GPIOExt;
use hal::systick::SystickDelay;
use hal::systick::SystickExt;
use hal::watchdog::WatchdogExt;
use hal::Oscillator;
use slstk3400a::leds::LedTrait;
use slstk3400a::SlStk3400a;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let hfrco = hal::HFRCO::init();
    hfrco.enable();
    let systick = cp.SYST.constrain();
    let systick_delay = SystickDelay::new(systick, hfrco.freq());
    let mut delay = Delay::new(CountProvider::SysTick(systick_delay));

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    let gpio = p.GPIO.constrain().split();
    let mut board = SlStk3400a::new(gpio).unwrap();
    let leds: [&mut dyn LedTrait; 2] = [&mut board.leds.led0, &mut board.leds.led1];

    let mut count = 0usize;
    loop {
        leds[count & 1].on();
        delay.delay_ms(1000u16);

        defmt::info!("Hello, world #{}!", count);

        leds[count & 1].off();
        delay.delay_ms(1000u16);

        count = count.wrapping_add(1);
    }
}
