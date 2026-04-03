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
use hal::clocks::enable_gpio_clock;
use hal::clocks::get_clock_config;
use hal::gpio::GPIOExt;
use hal::systick::SystickExt;
use hal::time_util::Hertz;
use hal::watchdog::WatchdogExt;
use portable_atomic::{AtomicUsize, Ordering};
use rt::exception;
use slstk3400a::SlStk3400a;

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));
static COUNT: AtomicUsize = AtomicUsize::new(0);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    // Enable GPIO clock to enable GPIO as outputs.
    enable_gpio_clock();

    let gpio = p.GPIO.constrain().split();
    let board = SlStk3400a::new(gpio).unwrap();
    critical_section::with(|lock| {
        BOARD.borrow(lock).replace(Some(board));
    });

    let mut systick = cp.SYST;
    systick.set_clock_source(SystClkSource::Core);
    let clock_config = get_clock_config().expect("Must be able to get clock config");
    let mut systick = systick.constrain(&clock_config);
    systick.enable_interrupt(Hertz(1000));

    loop {
        cortex_m::asm::wfe();
    }
}

#[exception]
fn SysTick() {
    let count = COUNT.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
    if !count.is_multiple_of(1000) {
        return;
    }

    critical_section::with(|lock| {
        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            let seconds = count / 1000;
            let leds = board.leds_mut();

            defmt::info!("Hello, world {}!", seconds);

            leds[seconds & 1].on();
            leds[(seconds - 1) & 1].off();
        };
    });
}
