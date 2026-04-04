//! Read the on-chip temperature sensor via ADC0 and display the result on the
//! Sharp Memory LCD.  The reading updates every second.
//!
//! The EFM32HG internal temperature sensor is on ADC0 channel 8 (TEMP).
//! The factory calibration values stored in DEVINFO are used to convert the
//! raw ADC reading to degrees Celsius.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use hal::clocks::enable_gpio_clock;
use hal::watchdog::WatchdogExt;
use slstk3400a::display;

/// DEVINFO base address for EFM32HG.
const DEVINFO_BASE: u32 = 0x0FE0_81B0;

/// Read a 32-bit value from the DEVINFO page.
///
/// # Safety
///
/// `offset` must yield a valid DEVINFO register address when added to
/// `DEVINFO_BASE`.
fn devinfo_read(offset: u32) -> u32 {
    // SAFETY: DEVINFO addresses are read-only factory-programmed MMIO in the
    // EFM32 information block - always valid and always readable.
    unsafe { core::ptr::read_volatile((DEVINFO_BASE + offset) as *const u32) }
}

/// Read factory ADC calibration temperature (degrees C, from DEVINFO).
fn cal_temp() -> i32 {
    // DEVINFO offset 0x00 = CAL
    // Bits 23:16 = TEMP (temperature in C at which ADC was calibrated)
    let cal = devinfo_read(0x00);
    ((cal >> 16) & 0xFF) as i32
}

/// Read factory ADC calibration reading at reference temperature.
fn cal_adc_value() -> i32 {
    // DEVINFO offset 0x0C = ADC0CAL2
    // Bits 31:20 = TEMP1V25 (12-bit ADC reading at cal_temp with 1.25V ref)
    let cal = devinfo_read(0x0C);
    ((cal >> 20) & 0xFFF) as i32
}

/// Initialise ADC0 for single-conversion reads from the temperature sensor.
fn adc_init(cmu: &pac::Cmu, adc: &pac::Adc0) {
    // Enable ADC0 clock.
    cmu.hfperclken0()
        .modify(|_, w: &mut pac::cmu::hfperclken0::W| w.adc0().set_bit());

    // Warm-up mode: keep ADC reference warm between conversions.
    adc.ctrl().write(|w: &mut pac::adc0::ctrl::W| unsafe {
        w.warmupmode()
            .keepadcwarm()
            .timebase()
            .bits(14)
            .presc()
            .nodivision()
    });

    // Single conversion: input = TEMP (channel 8), reference = 1.25V, 12-bit.
    adc.singlectrl().write(|w: &mut pac::adc0::singlectrl::W| {
        unsafe { w.inputsel().bits(8) }; // TEMP
        w.ref_()._1v25().res()._12bit().adj().clear_bit()
    });
}

/// Trigger a single ADC conversion and return the 12-bit result.
fn adc_read(adc: &pac::Adc0) -> u16 {
    adc.cmd()
        .write(|w: &mut pac::adc0::cmd::W| w.singlestart().set_bit());
    while adc.status().read().singledv().bit_is_clear() {}
    adc.singledata().read().data().bits() as u16
}

/// Convert raw ADC reading to temperature in degrees Celsius (×10 for 1 decimal).
fn adc_to_temp_x10(raw: u16) -> i32 {
    // T = T_cal - (ADC_raw - ADC_cal) * V_ref / (4096 * slope)
    // For EFM32HG: slope ≈ -6.27 mV/°C → 1/slope ≈ -159.5 °C/V
    // With 1.25V reference and 12-bit ADC:
    //   LSB = 1.25 / 4096 = 0.000305 V
    //   delta_T = -(ADC_raw - ADC_cal) * 0.000305 / 0.00627
    //           = -(ADC_raw - ADC_cal) * 1.25 / (4096 * 0.00627)
    //           ≈ -(ADC_raw - ADC_cal) * 0.04866
    // For ×10 precision: delta_T_x10 = -(ADC_raw - ADC_cal) * 1250 / (4096 * 627 / 100)
    //                                 = -(raw - cal) * 125000 / 25681

    let t_cal = cal_temp();
    let adc_cal = cal_adc_value();
    let delta = raw as i32 - adc_cal;
    // ×10 temperature:
    t_cal * 10 - (delta * 12500) / 25681 * 10
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.constrain().disable();
    enable_gpio_clock();

    // Initialise display and ADC.
    display::init();
    adc_init(&p.cmu, &p.adc0);

    let mut vcom = false;
    display::clear(&mut vcom);

    display::draw_text(0, 0, "Temperature", &mut vcom);
    display::draw_text(1, 0, "Sensor (ADC0)", &mut vcom);
    display::draw_text(4, 0, "Raw ADC:", &mut vcom);
    display::draw_text(6, 0, "Temp:", &mut vcom);

    defmt::info!("ADC temperature demo started");

    loop {
        let raw = adc_read(&p.adc0);
        let temp_x10 = adc_to_temp_x10(raw);

        // Format raw ADC value.
        let mut buf = [0u8; 10];
        let raw_str = display::format_u32(raw as u32, &mut buf);
        let mut line = [b' '; 16];
        line[..raw_str.len()].copy_from_slice(raw_str);
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text(5, 0, s, &mut vcom);

        // Format temperature: integer part and one decimal.
        let mut tbuf = [0u8; 16];
        let negative = temp_x10 < 0;
        let abs_x10 = temp_x10.unsigned_abs();
        let integer = abs_x10 / 10;
        let frac = abs_x10 % 10;

        let mut pos = 0usize;
        if negative {
            tbuf[pos] = b'-';
            pos += 1;
        }
        let mut ibuf = [0u8; 10];
        let istr = display::format_u32(integer, &mut ibuf);
        tbuf[pos..pos + istr.len()].copy_from_slice(istr);
        pos += istr.len();
        tbuf[pos] = b'.';
        pos += 1;
        tbuf[pos] = b'0' + frac as u8;
        pos += 1;
        tbuf[pos] = b' ';
        pos += 1;
        tbuf[pos] = b'C';
        pos += 1;

        // Pad remainder with spaces.
        while pos < 16 {
            tbuf[pos] = b' ';
            pos += 1;
        }

        let ts = core::str::from_utf8(&tbuf[..16]).unwrap_or("");
        display::draw_text(7, 0, ts, &mut vcom);

        defmt::info!("ADC raw={} temp={}  (x10={})", raw, temp_x10 / 10, temp_x10);

        // Wait ~1 second.
        cortex_m::asm::delay(14_000_000);
        display::toggle_vcom();
    }
}
