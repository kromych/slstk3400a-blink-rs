//! This demo shows how to use capacitive touch sensing with the ACMP0
//! peripheral in capsense mode. The ACMP0 output oscillates at a frequency
//! proportional to the pad capacitance. A PRS channel routes the ACMP0 output
//! to TIMER0 which counts edges over a fixed measurement window. When a finger
//! touches a pad the capacitance increases, the oscillation slows, and the
//! count drops below a threshold - indicating a touch.
//!
//! On the SLSTK3400A the two capacitive touch pads are:
//!   BUTTON0 → ACMP0 channel 4
//!   BUTTON1 → ACMP0 channel 3
//! (per capsenseconfig.h from the SiLabs board support package).
//!
//! This demo scans both pads and toggles the corresponding LED when touched.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use hal::clocks::enable_gpio_clock;
use hal::clocks::get_clock_config;
use hal::gpio::GPIOExt;
use hal::systick::SystickExt;
use hal::watchdog::WatchdogExt;
use slstk3400a::leds::{LedTrait, LEDs};

/// Number of capacitive touch channels to scan.
const NUM_CHANNELS: usize = 2;

/// Touch detection threshold - counts below this percentage of baseline indicate a touch.
const TOUCH_THRESHOLD_PCT: u32 = 80;

/// Measurement window in microseconds for each channel scan.
const MEASURE_WINDOW_US: u32 = 1000;

/// ACMP0 channel numbers for the two capsense pads (from capsenseconfig.h).
const CAPSENSE_CHANNELS: [u8; NUM_CHANNELS] = [4, 3]; // BUTTON0=ch4, BUTTON1=ch3

/// Set up ACMP0 for capacitive sense on the selected channel.
fn acmp_setup_capsense(acmp: &pac::Acmp0, channel: u8) {
    // Disable while reconfiguring.
    acmp.ctrl()
        .modify(|_, w: &mut pac::acmp0::ctrl::W| w.en().clear_bit());

    // Select channel and capsense mode.
    acmp.inputsel().write(|w: &mut pac::acmp0::inputsel::W| unsafe {
        w.possel().bits(channel);
        w.negsel().capsense().csresen().set_bit().csressel().res3()
    });

    // Enable ACMP with settings matching SiLabs capsense driver:
    // fullbias=1, biasprog=0x7, warmtime=512cycles, hystsel=hyst5.
    acmp.ctrl().write(|w: &mut pac::acmp0::ctrl::W| unsafe {
        w.en()
            .set_bit()
            .warmtime()
            ._512cycles()
            .hystsel()
            .hyst5()
            .fullbias()
            .set_bit()
            .biasprog()
            .bits(0x7)
    });
}

/// Measure the capsense oscillation count for one channel over `window_us` microseconds.
fn measure_channel(
    acmp: &pac::Acmp0,
    timer: &pac::Timer0,
    systick: &mut hal::systick::Systick,
    channel: u8,
    window_us: u32,
) -> u32 {
    acmp_setup_capsense(acmp, channel);

    // Wait for ACMP warm-up.
    while acmp.status().read().acmpact().bit_is_clear() {}

    // Reset TIMER0 counter and start.
    timer
        .cnt()
        .write(|w: &mut pac::timer0::cnt::W| unsafe { w.cnt().bits(0) });
    timer
        .cmd()
        .write(|w: &mut pac::timer0::cmd::W| w.start().set_bit());

    // Gate the measurement for the configured window.
    systick.delay_us(window_us).ok();

    // Stop timer.
    timer
        .cmd()
        .write(|w: &mut pac::timer0::cmd::W| w.stop().set_bit());

    timer.cnt().read().cnt().bits() as u32
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // Disable watchdog.
    p.wdog.constrain().disable();

    // Enable peripheral clocks: GPIO, ACMP0, TIMER0.
    enable_gpio_clock();
    p.cmu
        .hfperclken0()
        .modify(|_, w: &mut pac::cmu::hfperclken0::W| w.acmp0().set_bit().timer0().set_bit());

    // GPIO and LEDs.
    let gpio = p.gpio.constrain().split();
    let mut leds = LEDs::new(gpio.pf4.into(), gpio.pf5.into());
    leds.led0.off();
    leds.led1.off();

    // --- PRS: Route ACMP0 output (signal 0) to PRS channel 0 ---
    p.prs
        .ch0_ctrl()
        .write(|w: &mut pac::prs::ch0_ctrl::W| unsafe {
            w.sourcesel().acmp0().sigsel().bits(0); // ACMP0OUT
            w.edsel().posedge()
        });

    // --- TIMER0: Count PRS channel 0 edges via CC1 clock-select path ---
    // On EFM32HG, TIMER CLKSEL=CC1 means the timer is clocked by CC channel 1
    // input. We route PRS CH0 to CC1 input.
    p.timer0
        .cc1_ctrl()
        .write(|w: &mut pac::timer0::cc1_ctrl::W| {
            w.mode()
                .inputcapture()
                .insel()
                .set_bit() // PRS input
                .prssel()
                .prsch0()
                .icedge()
                .both()
        });

    p.timer0
        .ctrl()
        .write(|w: &mut pac::timer0::ctrl::W| w.clksel().cc1());
    p.timer0
        .top()
        .write(|w: &mut pac::timer0::top::W| unsafe { w.top().bits(0xFFFF) });

    // Set up SysTick for measurement gating.
    let mut systick = cp.SYST;
    systick.set_clock_source(SystClkSource::Core);
    let clock_config = get_clock_config().expect("Must be able to get clock config");
    let mut systick = systick.constrain(&clock_config);

    defmt::info!("Capsense demo started - BUTTON0=ch4, BUTTON1=ch3");

    // Establish baselines.
    let mut baselines = [0u32; NUM_CHANNELS];
    for (i, baseline) in baselines.iter_mut().enumerate() {
        let ch = CAPSENSE_CHANNELS[i];
        let mut sum = 0u32;
        for _ in 0..4 {
            sum += measure_channel(&p.acmp0, &p.timer0, &mut systick, ch, MEASURE_WINDOW_US);
        }
        *baseline = sum / 4;
        defmt::info!("Baseline BUTTON{} (ch{}): {}", i, ch, *baseline);
    }

    // Main scan loop.
    loop {
        for (i, baseline) in baselines.iter().enumerate() {
            let ch = CAPSENSE_CHANNELS[i];
            let count =
                measure_channel(&p.acmp0, &p.timer0, &mut systick, ch, MEASURE_WINDOW_US);

            let threshold = baseline * TOUCH_THRESHOLD_PCT / 100;
            let touched = count < threshold;

            if touched {
                defmt::info!("Touch BUTTON{} (ch{}): count={} baseline={}", i, ch, count, baseline);
            }

            // BUTTON0 (ch4) → LED 0, BUTTON1 (ch3) → LED 1.
            match i {
                0 => {
                    if touched {
                        leds.led0.on();
                    } else {
                        leds.led0.off();
                    }
                }
                1 => {
                    if touched {
                        leds.led1.on();
                    } else {
                        leds.led1.off();
                    }
                }
                _ => {}
            }
        }
    }
}
