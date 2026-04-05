//! Stopwatch demo for the SLSTK3400A board.
//!
//! Uses the RTC at 32768 Hz for accurate timekeeping and the Sharp Memory LCD
//! to display elapsed time in MM:SS.d format (1/10 second resolution).
//!
//! Controls:
//!   BTN0 (PC9)  - Start / Stop
//!   BTN1 (PC10) - Lap (while running) / Reset (while stopped)
//!
//! Up to 4 lap times are shown on screen.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use crate::pac::interrupt;
use core::cell::RefCell;
use critical_section::Mutex;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use hal::clocks::enable_gpio_clock;
use hal::gpio::ExtInterruptEdge;
use hal::gpio::GPIOExt;
use hal::gpio::GpioInterruptExt;
use hal::rtc::RTCExt;
use hal::watchdog::WatchdogExt;
use portable_atomic::{AtomicBool, AtomicU32, Ordering};
use slstk3400a::display;
use slstk3400a::leds::{LEDs, LedTrait};

static RTC: Mutex<RefCell<Option<hal::rtc::RTC>>> = Mutex::new(RefCell::new(None));

/// RTC ticks since stopwatch started (at 32768 Hz, wraps at ~36 hours).
static TICKS: AtomicU32 = AtomicU32::new(0);

/// Button event flags.
static BTN0_PRESSED: AtomicBool = AtomicBool::new(false);
static BTN1_PRESSED: AtomicBool = AtomicBool::new(false);

/// RTC compare value for 1/10 second ticks (32768 / 10 = 3277, close enough).
const TENTH_SEC_TICKS: u32 = 3277;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.constrain().disable();
    enable_gpio_clock();

    // --- Display ---
    display::init();
    let mut vcom = false;
    display::clear(&mut vcom);

    // --- GPIO: buttons ---
    let gpio = p.gpio.constrain().split();
    let mut btn0 = gpio.pc9.into_input_pulled_up();
    let mut btn1 = gpio.pc10.into_input_pulled_up();
    btn0.enable_interrupt(ExtInterruptEdge::Fall);
    btn1.enable_interrupt(ExtInterruptEdge::Fall);

    pac::NVIC::unpend(pac::Interrupt::GPIO_ODD);
    pac::NVIC::unpend(pac::Interrupt::GPIO_EVEN);
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIO_ODD);
        pac::NVIC::unmask(pac::Interrupt::GPIO_EVEN);
    }

    // --- LEDs ---
    let mut leds = LEDs::new(gpio.pf4.into(), gpio.pf5.into());
    leds.led0.off();
    leds.led1.off();

    // --- RTC: 32768 Hz, compare at TENTH_SEC_TICKS for 0.1s resolution ---
    p.cmu
        .hfcoreclken0()
        .write(|w: &mut pac::cmu::hfcoreclken0::W| w.le().set_bit());
    p.cmu
        .oscencmd()
        .write(|w: &mut pac::cmu::oscencmd::W| w.lfrcoen().set_bit());
    p.cmu.lfapresc0().reset();
    p.cmu
        .lfclksel()
        .write(|w: &mut pac::cmu::lfclksel::W| w.lfa().lfrco());
    p.cmu
        .lfaclken0()
        .write(|w: &mut pac::cmu::lfaclken0::W| w.rtc().set_bit());

    let mut rtc = p.rtc.constrain();
    rtc.reset();
    rtc.cap_counter(TENTH_SEC_TICKS, true);

    pac::NVIC::unpend(pac::Interrupt::RTC);
    unsafe { pac::NVIC::unmask(pac::Interrupt::RTC) };

    critical_section::with(|lock| {
        RTC.borrow(lock).replace(Some(rtc));
    });

    // --- State ---
    let mut running = false;
    let mut tenths: u32 = 0; // elapsed time in 1/10ths of a second
    let mut laps: [u32; 4] = [0; 4]; // lap times in tenths
    let mut lap_count: usize = 0;
    let mut display_dirty = true;

    // Draw static UI.
    display::draw_text(0, 0, "  STOPWATCH", &mut vcom);
    display::draw_text(2, 0, "BTN0:Start/Stop", &mut vcom);
    display::draw_text(3, 0, "BTN1: Lap/Reset", &mut vcom);
    draw_time(5, 0, &mut vcom);
    display::draw_text(7, 0, "  [STOPPED]", &mut vcom);

    defmt::info!("Stopwatch demo started");

    loop {
        cortex_m::asm::wfe();

        // Accumulate RTC ticks.
        let new_ticks = TICKS.swap(0, Ordering::Relaxed);
        if running && new_ticks > 0 {
            tenths += new_ticks;
            display_dirty = true;
        }

        // BTN0: start/stop.
        if BTN0_PRESSED.swap(false, Ordering::Relaxed) {
            running = !running;
            if running {
                // Start the RTC.
                critical_section::with(|lock| {
                    if let Some(rtc) = RTC.borrow(lock).borrow_mut().as_mut() {
                        rtc.start();
                    }
                });
                leds.led0.on();
                display::draw_text(7, 0, "  [RUNNING] ", &mut vcom);
            } else {
                // Stop the RTC.
                critical_section::with(|lock| {
                    if let Some(rtc) = RTC.borrow(lock).borrow_mut().as_mut() {
                        rtc.stop();
                    }
                });
                leds.led0.off();
                display::draw_text(7, 0, "  [STOPPED] ", &mut vcom);
            }
            display_dirty = true;
            // Debounce.
            cortex_m::asm::delay(2_800_000); // ~200ms
        }

        // BTN1: lap (if running) or reset (if stopped).
        if BTN1_PRESSED.swap(false, Ordering::Relaxed) {
            if running {
                // Record lap.
                if lap_count < 4 {
                    laps[lap_count] = tenths;
                    lap_count += 1;
                } else {
                    // Shift laps up and record newest.
                    laps[0] = laps[1];
                    laps[1] = laps[2];
                    laps[2] = laps[3];
                    laps[3] = tenths;
                }
                leds.led1.on();
                cortex_m::asm::delay(1_400_000); // flash ~100ms
                leds.led1.off();
            } else {
                // Reset.
                tenths = 0;
                lap_count = 0;
                laps = [0; 4];
                // Clear lap display.
                for r in 9..13 {
                    display::draw_text(r, 0, "                ", &mut vcom);
                }
            }
            display_dirty = true;
            cortex_m::asm::delay(2_800_000); // debounce
        }

        if display_dirty {
            display_dirty = false;

            // Draw main time.
            draw_time(5, tenths, &mut vcom);

            // Draw laps.
            if lap_count > 0 {
                display::draw_text(8, 0, "Laps:", &mut vcom);
                for (i, &lap) in laps.iter().enumerate().take(lap_count.min(4)) {
                    draw_lap(9 + i as u8, i as u8, lap, &mut vcom);
                }
            }

            // Toggle VCOM.
            display::toggle_vcom();
        }
    }
}

/// Format tenths of a second as "  MM:SS.d" and draw at the given text row.
fn draw_time(row: u8, tenths: u32, vcom: &mut bool) {
    let total_secs = tenths / 10;
    let frac = tenths % 10;
    let mins = total_secs / 60;
    let secs = total_secs % 60;

    let mut buf = [b' '; 16];
    // "  MM:SS.d"
    buf[2] = b'0' + (mins / 10 % 10) as u8;
    buf[3] = b'0' + (mins % 10) as u8;
    buf[4] = b':';
    buf[5] = b'0' + (secs / 10) as u8;
    buf[6] = b'0' + (secs % 10) as u8;
    buf[7] = b'.';
    buf[8] = b'0' + frac as u8;

    let s = core::str::from_utf8(&buf).unwrap_or("");
    display::draw_text(row, 0, s, vcom);
}

/// Draw a lap entry: "L#  MM:SS.d"
fn draw_lap(row: u8, idx: u8, tenths: u32, vcom: &mut bool) {
    let total_secs = tenths / 10;
    let frac = tenths % 10;
    let mins = total_secs / 60;
    let secs = total_secs % 60;

    let mut buf = [b' '; 16];
    buf[0] = b'L';
    buf[1] = b'1' + idx;
    buf[3] = b'0' + (mins / 10 % 10) as u8;
    buf[4] = b'0' + (mins % 10) as u8;
    buf[5] = b':';
    buf[6] = b'0' + (secs / 10) as u8;
    buf[7] = b'0' + (secs % 10) as u8;
    buf[8] = b'.';
    buf[9] = b'0' + frac as u8;

    let s = core::str::from_utf8(&buf).unwrap_or("");
    display::draw_text(row, 0, s, vcom);
}

// --- Interrupt handlers ---

#[interrupt]
fn RTC() {
    TICKS.fetch_add(1, Ordering::Relaxed);
    critical_section::with(|lock| {
        if let Some(rtc) = RTC.borrow(lock).borrow_mut().as_mut() {
            rtc.clear_all_interrupts();
        }
    });
    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_ODD() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc()
        .write(|w: &mut pac::gpio::ifc::W| unsafe { w.bits(1 << 9) });
    BTN0_PRESSED.store(true, Ordering::Relaxed);
    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_EVEN() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc()
        .write(|w: &mut pac::gpio::ifc::W| unsafe { w.bits(1 << 10) });
    BTN1_PRESSED.store(true, Ordering::Relaxed);
    cortex_m::asm::sev();
}
