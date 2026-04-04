//! Use GPIO interrupts to count push-button presses.
//!
//! BTN0 (PC9) triggers the GPIO_ODD interrupt on the falling edge (button
//! press). The running total is reported via defmt and toggled on the LEDs.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use crate::pac::interrupt;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use embedded_hal::digital::OutputPin;
use hal::clocks::enable_gpio_clock;
use hal::gpio::ExtInterruptEdge;
use hal::gpio::GPIOExt;
use hal::gpio::GpioInterruptExt;
use hal::watchdog::WatchdogExt;
use portable_atomic::{AtomicU32, Ordering};

static COUNT: AtomicU32 = AtomicU32::new(0);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.constrain().disable();
    enable_gpio_clock();

    let gpio = p.gpio.constrain().split();

    // LEDs.
    let mut led0 = gpio.pf4.into_pushpull();
    let mut led1 = gpio.pf5.into_pushpull();
    led0.set_low().ok();
    led1.set_low().ok();

    // BTN0 on PC9 — configure as input with pull-up and enable GPIO interrupt
    // on falling edge (button press).
    let mut btn0 = gpio.pc9.into_input_pulled_up();
    btn0.enable_interrupt(ExtInterruptEdge::Fall);

    pac::NVIC::unpend(pac::Interrupt::GPIO_ODD);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_ODD) };

    defmt::info!("Pulse counter demo — press BTN0 (PC9)");

    let mut last = u32::MAX;
    loop {
        let c = COUNT.load(Ordering::Relaxed);
        if c != last {
            last = c;
            defmt::info!("Button presses: {}", c);

            // Toggle LEDs based on count parity.
            if c & 1 == 0 {
                led0.set_high().ok();
                led1.set_low().ok();
            } else {
                led0.set_low().ok();
                led1.set_high().ok();
            }
        }
        cortex_m::asm::wfe();
    }
}

#[interrupt]
fn GPIO_ODD() {
    // Clear PC9 interrupt flag via the PAC.
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc()
        .write(|w: &mut pac::gpio::ifc::W| unsafe { w.bits(1 << 9) });

    COUNT.fetch_add(1, Ordering::Relaxed);
    cortex_m::asm::sev();
}
