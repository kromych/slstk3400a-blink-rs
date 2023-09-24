#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use crate::pac::interrupt;
use core::cell::RefCell;
use core::ops::DerefMut;
use critical_section::Mutex;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use embedded_hal::watchdog::WatchdogDisable;
use hal::gpio::ExtInterruptEdge;
use hal::gpio::GPIOExt;
use hal::oscillator::Clocks;
use hal::rtc::RTCExt;
use hal::watchdog::WatchdogExt;
use slstk3400a::SlStk3400a;

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    // Enable GPIO interrupts.
    pac::NVIC::unpend(pac::Interrupt::GPIO_EVEN);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_EVEN) };
    pac::NVIC::unpend(pac::Interrupt::GPIO_ODD);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_ODD) };

    // Enable GPIO clock to enable GPIO as outputs.
    let clks = Clocks::init();
    clks.enable_gpio_clock();

    // Board and GPIO.
    let gpio = p.GPIO.constrain().split();
    let mut board = SlStk3400a::new(gpio).unwrap();
    board.btns.btn0.0.enable_interrupt(ExtInterruptEdge::Fall);
    board.btns.btn1.0.enable_interrupt(ExtInterruptEdge::Fall);

    critical_section::with(|lock| {
        BOARD.borrow(lock).replace(Some(board));
    });

    loop {
        cortex_m::asm::wfe();
    }
}

/// Interrupt handler for RTC events (comp0 match).
#[interrupt]
fn GPIO_EVEN() {
    critical_section::with(|lock| {
        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            board.btns.btn0.0.clear_interrupt(0x5555);

            defmt::info!("even");

            let leds = board.leds_mut();
            leds[0].on();
            leds[1].off();
        };
    });
}

#[interrupt]
fn GPIO_ODD() {
    critical_section::with(|lock| {
        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            board.btns.btn1.0.clear_interrupt(0xAAAA);

            defmt::info!("odd");

            let leds = board.leds_mut();
            leds[1].on();
            leds[0].off();
        };
    });
}
