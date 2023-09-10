#![no_std]

use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use hal::cmu::CMUExt;
use hal::delay::CountProvider;
use hal::delay::Delay;
use hal::gpio::GPIOExt;
use hal::systick::SystickDelay;
use hal::systick::SystickExt;
use hal::watchdog::Watchdog;
use hal::watchdog::WatchdogExt;
use leds::LEDs;
use pac::CorePeripherals;
use pac::Peripherals;

pub mod leds;

pub struct SlStk3400a {
    pub leds: LEDs,
    pub delay: Delay,
    pub watchdog: Watchdog,
}

impl SlStk3400a {
    pub fn new() -> Option<Self> {
        let p = Peripherals::take()?;
        let cp = CorePeripherals::take()?;

        let clocks = p.CMU.constrain().freeze();
        let systick_delay = SystickDelay::new(cp.SYST.constrain(), &clocks);
        let gpio = p.GPIO.constrain(clocks.gpio).split();
        Some(Self {
            watchdog: p.WDOG.constrain(),
            leds: LEDs::new(gpio.pf4.into(), gpio.pf5.into()),
            delay: Delay::new(CountProvider::SysTick(systick_delay)),
        })
    }
}
