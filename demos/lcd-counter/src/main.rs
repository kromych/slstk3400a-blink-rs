//! Count seconds using the RTC and display the value on the Sharp Memory LCD.
//! Push button 0 (PC9) resets the counter.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use crate::pac::interrupt;
use core::cell::RefCell;
use portable_atomic::{AtomicBool, AtomicU32, Ordering};
use critical_section::Mutex;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use embedded_hal::watchdog::WatchdogDisable;
use hal::clocks::enable_gpio_clock;
use hal::gpio::ExtInterruptEdge;
use hal::gpio::GPIOExt;
use hal::gpio::GpioInterruptExt;
use hal::rtc::RTCExt;
use hal::watchdog::WatchdogExt;
use slstk3400a::display;

static RTC: Mutex<RefCell<Option<hal::rtc::RTC>>> = Mutex::new(RefCell::new(None));
static SECONDS: AtomicU32 = AtomicU32::new(0);
static RESET_REQ: AtomicBool = AtomicBool::new(false);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.WDOG.constrain().disable();
    enable_gpio_clock();

    // Initialise the LCD.
    display::init();
    let mut vcom = false;
    display::clear(&mut vcom);

    display::draw_text(0, 0, "RTC Counter", &mut vcom);
    display::draw_text(2, 0, "Seconds:", &mut vcom);
    display::draw_text(6, 0, "BTN0 = reset", &mut vcom);

    // Configure push button 0 (PC9) for interrupt on press.
    let gpio = p.GPIO.constrain().split();
    let mut btn0 = gpio.pc9.into_input_pulled_up();
    btn0.enable_interrupt(ExtInterruptEdge::Fall);

    pac::NVIC::unpend(pac::Interrupt::GPIO_ODD);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_ODD) };

    // Configure RTC: LFRCO 32768 Hz, compare at 32768 → 1-second tick.
    p.CMU.hfcoreclken0().write(|w| w.le().set_bit());
    p.CMU.oscencmd().write(|w| w.lfrcoen().set_bit());
    p.CMU.lfapresc0().reset();
    p.CMU.lfclksel().write(|w| w.lfa().lfrco());
    p.CMU.lfaclken0().write(|w| w.rtc().set_bit());

    let mut rtc = p.RTC.constrain();
    rtc.reset();
    rtc.cap_counter(32_768, true);

    pac::NVIC::unpend(pac::Interrupt::RTC);
    unsafe { pac::NVIC::unmask(pac::Interrupt::RTC) };

    critical_section::with(|lock| {
        RTC.borrow(lock).replace(Some(rtc));
    });
    critical_section::with(|lock| {
        if let Some(rtc) = RTC.borrow(lock).borrow_mut().as_mut() {
            rtc.start();
        }
    });

    let mut last_displayed = u32::MAX;
    loop {
        // Check for counter reset request from button ISR.
        if RESET_REQ.swap(false, Ordering::Relaxed) {
            SECONDS.store(0, Ordering::Relaxed);
        }

        let secs = SECONDS.load(Ordering::Relaxed);
        if secs != last_displayed {
            last_displayed = secs;

            // Format the number and display it.
            let mut buf = [0u8; 10];
            let digits = display::format_u32(secs, &mut buf);
            // Pad to 10 chars for clean overwrite.
            let mut line = [b' '; 16];
            line[..digits.len()].copy_from_slice(digits);

            // SAFETY: line is valid ASCII.
            let s = core::str::from_utf8(&line).unwrap_or("");
            display::draw_text(3, 0, s, &mut vcom);

            defmt::info!("Counter: {}", secs);
        }

        // Toggle VCOM (display maintenance).
        display::toggle_vcom();

        cortex_m::asm::wfe();
    }
}

#[interrupt]
fn RTC() {
    SECONDS.fetch_add(1, Ordering::Relaxed);
    critical_section::with(|lock| {
        if let Some(rtc) = RTC.borrow(lock).borrow_mut().as_mut() {
            rtc.clear_all_interrupts();
        }
    });
    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_ODD() {
    // Clear PC9 interrupt flag.
    let gpio = unsafe { &*pac::GPIO::ptr() };
    gpio.ifc().write(|w| unsafe { w.bits(1 << 9) });

    RESET_REQ.store(true, Ordering::Relaxed);
    cortex_m::asm::sev();
}
