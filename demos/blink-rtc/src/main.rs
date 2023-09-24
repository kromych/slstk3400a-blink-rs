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
use hal::watchdog::WatchdogExt;
use slstk3400a::SlStk3400a;

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));

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
    p.RTC.freeze.reset();
    p.RTC.ctrl.reset();
    p.RTC.ien.reset();
    p.RTC
        .ifc
        .write(|w| w.comp0().set_bit().comp1().set_bit().of().set_bit());
    p.RTC.comp0.reset();
    p.RTC.comp1.reset();

    // Interrupt when matching custom compare value:
    // 65536 / 32768 Hz = 2 secs. Can request an interrupt on overflow.
    p.RTC.comp0.write(|w| unsafe { w.comp0().bits(65_536) });
    p.RTC.ien.modify(|_, w| w.comp0().set_bit());

    // Cap counter at `comp0` value.
    p.RTC.ctrl.modify(|_, w| w.comp0top().set_bit());

    // Enable RTC interrupts.
    pac::NVIC::unpend(pac::Interrupt::RTC);
    unsafe { pac::NVIC::unmask(pac::Interrupt::RTC) };

    // Board and GPIO.
    let gpio = p.GPIO.constrain().split();
    let board = SlStk3400a::new(gpio).unwrap();
    critical_section::with(|lock| {
        BOARD.borrow(lock).replace(Some(board));
    });

    // Start RTC.
    p.RTC.ctrl.modify(|_, w| w.en().set_bit());

    loop {
        cortex_m::asm::wfe();
    }
}

/// Interrupt handler for RTC events (comp0 match).
#[interrupt]
fn RTC() {
    let rtc = unsafe { &*pac::RTC::ptr() };
    critical_section::with(|lock| {
        // Clear interrupt.
        rtc.ifc.write(|w| w.comp0().set_bit());

        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            let _leds = board.leds_mut();

            defmt::info!("Hello, world!");
        };
    });
}
