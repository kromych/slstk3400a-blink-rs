//! Space Invaders demo for the SLSTK3400A board.
//!
//! Uses the 128x128 Sharp Memory LCD, two push buttons (left/right),
//! and TIMER0 for a ~15 Hz game tick.

#![no_main]
#![no_std]

mod game;
mod render;
mod rng;
mod sprites;

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
use hal::timer::{InterruptFlag, TimerExt};
use hal::watchdog::WatchdogExt;
use portable_atomic::{AtomicBool, Ordering};
use slstk3400a::display;
use slstk3400a::leds::{LEDs, LedTrait};

use game::GameState;

// --- Shared state between ISRs and main loop ---

/// Game tick flag, set by TIMER0 overflow ISR.
static TICK: AtomicBool = AtomicBool::new(false);
/// Button states, updated by GPIO ISRs (active-low: pressed when pin reads 0).
static BTN_LEFT: AtomicBool = AtomicBool::new(false);
static BTN_RIGHT: AtomicBool = AtomicBool::new(false);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.constrain().disable();
    enable_gpio_clock();

    // --- Display init ---
    display::init();
    #[cfg(feature = "dma")]
    display::dma_init();
    let mut vcom = false;
    display::clear(&mut vcom);

    // --- GPIO: buttons with interrupts ---
    let gpio = p.gpio.constrain().split();
    let mut btn0 = gpio.pc9.into_input_pulled_up();
    let mut btn1 = gpio.pc10.into_input_pulled_up();
    btn0.enable_interrupt(ExtInterruptEdge::Both);
    btn1.enable_interrupt(ExtInterruptEdge::Both);

    pac::NVIC::unpend(pac::Interrupt::GPIO_ODD);
    pac::NVIC::unpend(pac::Interrupt::GPIO_EVEN);
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIO_ODD);
        pac::NVIC::unmask(pac::Interrupt::GPIO_EVEN);
    }

    // --- LEDs (for feedback) ---
    let mut leds = LEDs::new(gpio.pf4.into(), gpio.pf5.into());

    // --- TIMER0: ~15 Hz game tick ---
    // Enable TIMER0 clock.
    p.cmu
        .hfperclken0()
        .modify(|_, w: &mut pac::cmu::hfperclken0::W| w.timer0().set_bit());

    // Default HFRCO is 14 MHz. With prescaler /1024, base = ~13672 Hz.
    // TOP = 910 → 13672/911 ≈ 15.0 Hz.
    // Set prescaler to /1024 via CTRL register before consuming the peripheral.
    p.timer0
        .ctrl()
        .write(|w: &mut pac::timer0::ctrl::W| w.presc().div1024());

    let mut timer = p.timer0.with_clock();

    timer.set_top(910);
    timer.interrupt_enable(InterruptFlag::OF);

    pac::NVIC::unpend(pac::Interrupt::TIMER0);
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0) };

    timer.start();

    // --- Title screen ---
    render::fb_clear();
    render::fb_draw_text(4, 1, "SPACE INVADERS");
    render::fb_draw_text(8, 2, "BTN0: LEFT");
    render::fb_draw_text(9, 2, "BTN1: RIGHT");
    render::fb_draw_text(11, 2, "BOTH: FIRE!");
    render::fb_draw_text(14, 2, "PRESS  BTN0");
    render::fb_flush(&mut vcom);

    // Wait for button press to start.
    loop {
        cortex_m::asm::wfe();
        if BTN_LEFT.load(Ordering::Relaxed) {
            break;
        }
    }

    // --- Game loop ---
    let mut state = GameState::new(0xBEEF);
    state.draw(&mut vcom);

    let mut vcom_counter: u8 = 0;

    loop {
        cortex_m::asm::wfe();

        if !TICK.swap(false, Ordering::Relaxed) {
            continue;
        }

        // Read button state.
        let left = BTN_LEFT.load(Ordering::Relaxed);
        let right = BTN_RIGHT.load(Ordering::Relaxed);
        let fire = left && right;

        // Update game.
        state.update(left, right, fire);

        // Render.
        state.draw(&mut vcom);

        // Toggle VCOM at ~1 Hz (every 15 ticks).
        vcom_counter += 1;
        if vcom_counter >= 15 {
            vcom_counter = 0;
            display::toggle_vcom();
        }

        // LED feedback.
        if fire {
            leds.led0.on();
        } else {
            leds.led0.off();
        }
        if state.game_over {
            leds.led1.on();
        }
    }
}

// --- Interrupt handlers ---

#[interrupt]
fn TIMER0() {
    hal::timer::Timer0::interrupt_unpend(InterruptFlag::OF);
    TICK.store(true, Ordering::Relaxed);
    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_ODD() {
    // PC9 (BTN0 = LEFT) - odd pin number.
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc()
        .write(|w: &mut pac::gpio::ifc::W| unsafe { w.bits(1 << 9) });

    let pressed = gpio.pc_din().read().bits() & (1 << 9) == 0;
    BTN_LEFT.store(pressed, Ordering::Relaxed);
    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_EVEN() {
    // PC10 (BTN1 = RIGHT) - even pin number.
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc()
        .write(|w: &mut pac::gpio::ifc::W| unsafe { w.bits(1 << 10) });

    let pressed = gpio.pc_din().read().bits() & (1 << 10) == 0;
    BTN_RIGHT.store(pressed, Ordering::Relaxed);
    cortex_m::asm::sev();
}
