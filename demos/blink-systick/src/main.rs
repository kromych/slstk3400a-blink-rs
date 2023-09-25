//! This demo shows how to blink an LED and use the SysTick interrupt.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as rt;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::peripheral::syst::SystClkSource;
use critical_section::Mutex;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use embedded_hal::watchdog::WatchdogDisable;
use hal::gpio::GPIOExt;
use hal::oscillator::ushfrco::USHFRCO_FREQUENCY;
use hal::oscillator::Clocks;
use hal::systick::SystickExt;
use hal::time_util::Hertz;
use hal::watchdog::WatchdogExt;
use hal::Oscillator;
use rt::exception;
use slstk3400a::SlStk3400a;

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    let osc = hal::USHFRCO::init();
    osc.enable();
    osc.select_hfclk();

    // Enable GPIO clock to enable GPIO as outputs.
    let clks = Clocks::init();
    clks.enable_gpio_clock();

    let gpio = p.GPIO.constrain().split();
    let board = SlStk3400a::new(gpio).unwrap();
    critical_section::with(|lock| {
        BOARD.borrow(lock).replace(Some(board));
    });

    let mut systick = cp.SYST;
    systick.set_clock_source(SystClkSource::Core);
    let mut systick = systick.constrain(Hertz(USHFRCO_FREQUENCY.0 / 2));
    systick.enable_interrupt(Hertz(1000));

    loop {
        cortex_m::asm::wfe();
    }
}

#[exception]
fn SysTick() {
    static mut COUNT: usize = 0;

    critical_section::with(|lock| {
        *COUNT = COUNT.wrapping_add(1);
        if *COUNT % 1000 != 0 {
            return;
        }

        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            let seconds = *COUNT / 1000;
            let leds = board.leds_mut();

            defmt::info!("Hello, world {}!", seconds);

            leds[seconds & 1].on();
            leds[(seconds - 1) & 1].off();
        };
    });
}
