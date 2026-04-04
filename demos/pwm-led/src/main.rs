//! Software PWM LED "breathing" demo for the SLSTK3400A board.
//!
//! Uses TIMER2 overflow interrupt at ~10 kHz to bit-bang PWM on the two LEDs
//! (PF4, PF5). The duty cycle ramps up and down smoothly, creating a breathing
//! effect. The two LEDs are 180° out of phase.
//!
//! BTN0 (PC9) pauses/resumes the animation.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use crate::pac::interrupt;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use hal::clocks::enable_gpio_clock;
use hal::gpio::ExtInterruptEdge;
use hal::gpio::GPIOExt;
use hal::gpio::GpioInterruptExt;
use hal::watchdog::WatchdogExt;
use portable_atomic::{AtomicBool, AtomicU8, Ordering};

/// PWM resolution (number of steps in one PWM period).
const PWM_STEPS: u8 = 100;

/// Current duty cycle for LED0 (0–100).
static DUTY: AtomicU8 = AtomicU8::new(0);
/// PWM tick counter (0–99), incremented by TIMER2 ISR.
static PWM_TICK: AtomicU8 = AtomicU8::new(0);
/// Pause flag, toggled by BTN0.
static PAUSED: AtomicBool = AtomicBool::new(false);

/// GPIO port F bit positions for the LEDs.
const PF_LED0: u32 = 1 << 4;
const PF_LED1: u32 = 1 << 5;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.constrain().disable();
    enable_gpio_clock();

    // Configure LED pins (PF4, PF5) as push-pull output.
    let gpio = p.gpio.constrain().split();
    let _led0 = gpio.pf4.into_pushpull();
    let _led1 = gpio.pf5.into_pushpull();

    // Configure BTN0 (PC9) with interrupt.
    let mut btn0 = gpio.pc9.into_input_pulled_up();
    btn0.enable_interrupt(ExtInterruptEdge::Fall);
    pac::NVIC::unpend(pac::Interrupt::GPIO_ODD);
    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_ODD) };

    // Enable TIMER2 clock.
    p.cmu
        .hfperclken0()
        .modify(|_, w: &mut pac::cmu::hfperclken0::W| w.timer2().set_bit());

    // Configure TIMER2 for ~10 kHz overflow (14 MHz / 1 / 1400 = 10 kHz).
    // 10 kHz / 100 steps = 100 Hz PWM frequency - flicker-free.
    p.timer2
        .ctrl()
        .write(|w: &mut pac::timer2::ctrl::W| w.presc().div1());
    p.timer2
        .top()
        .write(|w: &mut pac::timer2::top::W| unsafe { w.top().bits(1399) });

    // Enable overflow interrupt.
    p.timer2
        .ien()
        .write(|w: &mut pac::timer2::ien::W| w.of().set_bit());
    pac::NVIC::unpend(pac::Interrupt::TIMER2);
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER2) };

    // Start timer.
    p.timer2
        .cmd()
        .write(|w: &mut pac::timer2::cmd::W| w.start().set_bit());

    defmt::info!("PWM LED breathing demo started. BTN0 = pause/resume.");

    // Main loop: update duty cycle smoothly.
    let mut duty: u8 = 0;
    let mut rising = true;

    loop {
        cortex_m::asm::wfe();

        if PAUSED.load(Ordering::Relaxed) {
            continue;
        }

        // Ramp speed: update duty every ~10 ms (100 PWM cycles).
        // We use a simple software counter since we get ~100 WFE wakeups per second
        // from the 100 Hz PWM period completions.
        // Actually, the TIMER2 ISR fires at 10 kHz, so WFE wakes very often.
        // Use a delay instead.
        cortex_m::asm::delay(140_000); // ~10 ms at 14 MHz

        if rising {
            if duty < PWM_STEPS {
                duty += 1;
            } else {
                rising = false;
            }
        } else {
            if duty > 0 {
                duty -= 1;
            } else {
                rising = true;
            }
        }

        DUTY.store(duty, Ordering::Relaxed);
    }
}

#[interrupt]
fn TIMER2() {
    // Clear overflow flag.
    let timer = unsafe { &*pac::Timer2::ptr() };
    timer
        .ifc()
        .write(|w: &mut pac::timer2::ifc::W| w.of().set_bit());

    let tick = PWM_TICK.load(Ordering::Relaxed);
    let next = if tick + 1 >= PWM_STEPS { 0 } else { tick + 1 };
    PWM_TICK.store(next, Ordering::Relaxed);

    let duty = DUTY.load(Ordering::Relaxed);
    let gpio = unsafe { &*pac::Gpio::ptr() };

    // LED0: on when tick < duty (normal phase).
    // LED1: on when tick < (PWM_STEPS - duty) (inverted phase).
    if tick < duty {
        gpio.pf_doutset().write(|w| unsafe { w.bits(PF_LED0) });
    } else {
        gpio.pf_doutclr().write(|w| unsafe { w.bits(PF_LED0) });
    }

    let inv_duty = PWM_STEPS - duty;
    if tick < inv_duty {
        gpio.pf_doutset().write(|w| unsafe { w.bits(PF_LED1) });
    } else {
        gpio.pf_doutclr().write(|w| unsafe { w.bits(PF_LED1) });
    }

    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_ODD() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc()
        .write(|w: &mut pac::gpio::ifc::W| unsafe { w.bits(1 << 9) });

    // Toggle pause state.
    let was_paused = PAUSED.load(Ordering::Relaxed);
    PAUSED.store(!was_paused, Ordering::Relaxed);
    cortex_m::asm::sev();
}
