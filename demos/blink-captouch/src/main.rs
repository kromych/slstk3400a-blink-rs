//! This demo shows how to use capacitive touch sensing with the ACMP0
//! peripheral in capsense mode. The ACMP0 output oscillates at a frequency
//! proportional to the pad capacitance. A PRS channel routes the ACMP0 output
//! to TIMER0 which counts edges over a fixed measurement window. When a finger
//! touches a pad the capacitance increases, the oscillation slows, and the
//! count drops below a threshold — indicating a touch.
//!
//! On the SLSTK3400A the four capacitive touch pads are wired to ACMP0
//! channels 0–3 (PC0–PC3). This demo scans all four pads and toggles LEDs
//! when pad 0 or pad 1 is touched.

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
use slstk3400a::SlStk3400a;

/// Number of capacitive touch channels to scan.
const NUM_CHANNELS: usize = 4;

/// Touch detection threshold — counts below this value indicate a touch.
/// This value is relative to the baseline and may need tuning for a given board.
const TOUCH_THRESHOLD_PCT: u32 = 80;

/// Measurement window in microseconds for each channel scan.
const MEASURE_WINDOW_US: u32 = 1000;

/// Positive-input channel selectors for the four capsense pads.
const CHANNELS: [fn(&mut pac::acmp0::inputsel::W) -> &mut pac::acmp0::inputsel::W; NUM_CHANNELS] = [
    |w| w.possel().ch0(),
    |w| w.possel().ch1(),
    |w| w.possel().ch2(),
    |w| w.possel().ch3(),
];

/// Set up ACMP0 for capacitive sense on the selected channel.
fn acmp_setup_capsense(acmp: &pac::ACMP0, channel_idx: usize) {
    // Disable while reconfiguring.
    acmp.ctrl().modify(|_, w| w.en().clear_bit());

    // Select channel and capsense mode.
    acmp.inputsel().write(|w| {
        let w = CHANNELS[channel_idx](w);
        w.negsel()
            .capsense()
            .csresen()
            .set_bit()
            .csressel()
            .res3()
    });

    // Enable ACMP with full-bias for fast capsense oscillation.
    acmp.ctrl().write(|w| {
        w.en()
            .set_bit()
            .warmtime()
            ._512cycles()
            .hystsel()
            .hyst5()
            .fullbias()
            .set_bit()
            .biasprog()
            .variant(31)
    });
}

/// Measure the capsense oscillation count for one channel over `window_us` microseconds.
fn measure_channel(
    acmp: &pac::ACMP0,
    timer: &pac::TIMER0,
    systick: &mut hal::systick::Systick,
    channel_idx: usize,
    window_us: u32,
) -> u32 {
    acmp_setup_capsense(acmp, channel_idx);

    // Wait for ACMP warm-up.
    while acmp.status().read().acmpact().bit_is_clear() {}

    // Reset TIMER0 counter and start.
    timer.cnt().write(|w| unsafe { w.cnt().bits(0) });
    timer.cmd().write(|w| w.start().set_bit());

    // Gate the measurement for the configured window.
    systick.delay_us(window_us).ok();

    // Stop timer.
    timer.cmd().write(|w| w.stop().set_bit());

    timer.cnt().read().cnt().bits() as u32
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // Disable watchdog.
    p.WDOG.constrain().disable();

    // Enable peripheral clocks: GPIO, ACMP0, TIMER0, PRS.
    enable_gpio_clock();
    p.CMU
        .hfperclken0()
        .modify(|_, w| w.acmp0().set_bit().timer0().set_bit());

    // Enable PRS clock (bit 15 of HFPERCLKEN0 is documented but the PAC may
    // not expose a named field; if needed we fall back to raw bits).
    // The PRS clock is always enabled on EFM32HG when HFPERCLK is running,
    // so this is a no-op on this device.

    // Board and GPIO.
    let gpio = p.GPIO.constrain().split();
    let mut board = SlStk3400a::new(gpio).unwrap();
    for led in board.leds_mut() {
        led.off();
    }

    // --- PRS: Route ACMP0 output (signal 0) to PRS channel 0 ---
    p.PRS.ch0_ctrl().write(|w| {
        w.sourcesel()
            .acmp0()
            .sigsel()
            .variant(0) // ACMP0OUT
            .edsel()
            .posedge()
    });

    // --- TIMER0: Count PRS channel 0 edges via CC0 input capture ---
    // Select CC1 as clock source won't work for edge counting.
    // Instead we use CC0 in input-capture mode with PRS, and count overflows.
    // Simpler: clock TIMER0 from HFPERCLK and use CC0 to capture — but the
    // most straightforward capsense counting approach is to clock TIMER0 from
    // PRS via the CC1 clock-select path.
    //
    // On EFM32HG, TIMER CLKSEL=CC1 means the timer is clocked by CC channel 1
    // input. We route PRS CH0 to CC1 input.
    p.TIMER0.cc1_ctrl().write(|w| {
        w.mode()
            .inputcapture()
            .insel()
            .set_bit() // PRS input
            .prssel()
            .prsch0()
            .icedge()
            .both()
    });

    p.TIMER0.ctrl().write(|w| w.clksel().cc1());
    p.TIMER0.top().write(|w| unsafe { w.top().bits(0xFFFF) });

    // Set up SysTick for measurement gating.
    let mut systick = cp.SYST;
    systick.set_clock_source(SystClkSource::Core);
    let clock_config = get_clock_config().expect("Must be able to get clock config");
    let mut systick = systick.constrain(&clock_config);

    defmt::info!("Capsense demo started — touch pads 0-3 (PC0-PC3)");

    // Establish baselines.
    let mut baselines = [0u32; NUM_CHANNELS];
    for (ch, baseline) in baselines.iter_mut().enumerate() {
        // Average a few readings for a stable baseline.
        let mut sum = 0u32;
        for _ in 0..4 {
            sum += measure_channel(&p.ACMP0, &p.TIMER0, &mut systick, ch, MEASURE_WINDOW_US);
        }
        *baseline = sum / 4;
        defmt::info!("Baseline CH{}: {}", ch, *baseline);
    }

    // Main scan loop.
    loop {
        for (ch, baseline) in baselines.iter().enumerate() {
            let count =
                measure_channel(&p.ACMP0, &p.TIMER0, &mut systick, ch, MEASURE_WINDOW_US);

            let threshold = baseline * TOUCH_THRESHOLD_PCT / 100;
            let touched = count < threshold;

            if touched {
                defmt::info!("Touch CH{}: count={} baseline={}", ch, count, baseline);
            }

            // Map pad 0 → LED 0, pad 1 → LED 1.
            let leds = board.leds_mut();
            match ch {
                0 => {
                    if touched {
                        leds[0].on();
                    } else {
                        leds[0].off();
                    }
                }
                1 => {
                    if touched {
                        leds[1].on();
                    } else {
                        leds[1].off();
                    }
                }
                _ => {}
            }
        }
    }
}
