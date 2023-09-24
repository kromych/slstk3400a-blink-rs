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
use hal::gpio::GPIOExt;
use hal::oscillator::Clocks;
use hal::rtc::RTCExt;
use hal::watchdog::WatchdogExt;
use slstk3400a::SlStk3400a;

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));
static RTC: Mutex<RefCell<Option<hal::rtc::RTC>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG.constrain().disable();

    // Configure clock to RTC:
    //  * LFRCO ticks at 32768 Hz
    //  * No clock divider
    p.CMU.hfcoreclken0.write(|w| w.le().set_bit());
    p.CMU.oscencmd.write(|w| w.lfrcoen().set_bit());
    p.CMU.lfapresc0.reset();
    p.CMU.lfclksel.write(|w| w.lfa().lfrco());

    // Enable clock to RTC, ticking at 32 KiHz.
    p.CMU.lfaclken0.write(|w| w.rtc().set_bit());

    // Reset RTC
    let mut rtc = p.RTC.constrain();
    rtc.reset();

    // Interrupt when matching custom compare value:
    // 32768 / 32768 Hz = 1 secs. Request an interrupt.
    rtc.cap_counter(32_768, true);

    // Enable RTC interrupts.
    pac::NVIC::unpend(pac::Interrupt::RTC);
    unsafe { pac::NVIC::unmask(pac::Interrupt::RTC) };

    // Start RTC.
    critical_section::with(|lock| {
        RTC.borrow(lock).replace(Some(rtc));
    });
    critical_section::with(|lock| {
        if let Some(rtc) = RTC.borrow(lock).borrow_mut().deref_mut() {
            rtc.start();
        }
    });

    // Enable GPIO clock to enable GPIO as outputs.
    let clks = Clocks::init();
    clks.enable_gpio_clock();

    // Board and GPIO.
    let gpio = p.GPIO.constrain().split();
    let board = SlStk3400a::new(gpio).unwrap();
    critical_section::with(|lock| {
        BOARD.borrow(lock).replace(Some(board));
    });

    loop {
        cortex_m::asm::wfe();
    }
}

/// Interrupt handler for RTC events (comp0 match).
#[interrupt]
fn RTC() {
    static mut COUNT: usize = 0;

    critical_section::with(|lock| {
        *COUNT = COUNT.wrapping_add(1);

        if let Some(rtc) = RTC.borrow(lock).borrow_mut().deref_mut() {
            // Clear interrupt.
            rtc.clear_all_interrupts();
            if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
                let seconds = *COUNT;
                let leds = board.leds_mut();

                defmt::info!("Hello, world {}!", seconds);

                leds[seconds & 1].on();
                leds[(seconds - 1) & 1].off();
            };
        }
    });
}
