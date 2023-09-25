//! This demo shows how to process interrupts from GPIO pins
//! and blink an LED in the interrupt handler.

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
use hal::watchdog::WatchdogExt;
use slstk3400a::SlStk3400a;

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    // Enable GPIO clock to enable GPIO as outputs.
    let clks = Clocks::init();
    clks.enable_gpio_clock();

    // Board and GPIO.
    let gpio = p.GPIO.constrain().split();
    let mut board = SlStk3400a::new(gpio).unwrap();
    for led in board.leds_mut() {
        led.off();
    }
    for btn in board.btns_mut() {
        btn.enable_interrupt(ExtInterruptEdge::Fall);
    }

    critical_section::with(|lock| {
        BOARD.borrow(lock).replace(Some(board));
    });

    // Enable GPIO interrupts.
    pac::NVIC::unpend(pac::Interrupt::GPIO_EVEN);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_EVEN) };
    pac::NVIC::unpend(pac::Interrupt::GPIO_ODD);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_ODD) };

    loop {
        cortex_m::asm::wfi();
    }
}

/// Interrupt handler for the even-numbered GPIO pins (PC10 with the BTN1 in particular).
#[interrupt]
fn GPIO_EVEN() {
    // One has to make sure which GPIO pin changed state.
    // Here, the interruts are enabled for the two push buttons
    // one of which is on the odd GPIO pin, and the other one is
    // one the even GPIO pin so the check is skipped.

    defmt::debug!("even");

    critical_section::with(|lock| {
        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            board.btns_mut()[1].clear_interrupt();

            let leds = board.leds_mut();
            leds[1].on();
            leds[0].off();
        };
    });
}

/// Interrupt handler for the odd-numbered GPIO pins (PC9 with the BTN1 in particular).
#[interrupt]
fn GPIO_ODD() {
    // One has to make sure which GPIO pin changed state.
    // Here, the interruts are enabled for the two push buttons
    // one of which is on the odd GPIO pin, and the other one is
    // one the even GPIO pin so the check is skipped.

    defmt::debug!("odd");

    critical_section::with(|lock| {
        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            board.btns_mut()[0].clear_interrupt();

            let leds = board.leds_mut();
            leds[0].on();
            leds[1].off();
        };
    });
}
