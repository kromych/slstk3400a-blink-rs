use crate::time_util::Hertz;
use defmt::Format;
use efm32hg322_pac as pac;
use pac::cmu::hfrcoctrl;
use pac::devinfo::RegisterBlock;
use pac::msc::readctrl::MODE_A;

/// The hgh frequency crystal on SLSTK3400A.
pub const HFXO_FREQUENCY: Hertz = Hertz(24_000_000);
/// The low frequency crystal on SLSTK3400A.
pub const LFXO_FREQUENCY: Hertz = Hertz(32_768);
/// The high frequency RC oscillator.
pub const DEFAULT_HFRCO_FREQUENCY: Hertz = Hertz(14_000_000);
/// The low frequency RC oscillator.
pub const DEFAULT_LFRCO_FREQUENCY: Hertz = Hertz(32_768);

/// Threshold for inserting 1 wait state
const WAIT_STATE_1_THRESHOLD: u32 = 16_000_000;

/// Clock Source
#[derive(Clone, Copy, Format)]
pub enum ClockSource {
    None,
    /// Low Frequency Crystal Oscillator
    LFXO(Hertz),
    /// Low Frequency Internal RC Oscillator
    LFRCO(Hertz),
    /// High Frequency Internal RC Oscillator
    HFRCO(Hertz),
    /// High Frequency Crystall Oscillator
    HFXO(Hertz),
    /// Ultra Low Frequency
    ULFRCO(Hertz),
    /// HF Core Clock divided by 2
    HFCoreClock2(Hertz),
}

/// Clock Configuration
#[derive(Clone, Copy, Format)]
pub struct ClockConfiguration {
    /// Clock source
    pub source: ClockSource,
    /// Base frequency of core clock source
    pub basefreq: u32,
    /// Divisor of base frequency to generate HFCLK
    pub hclkdiv: u8,
    /// Divisor of HFCLK to generate Core Clock
    pub hfcoreclkdiv: u8,
    /// Divisor of HFCLK to generate Peripheral Clock
    pub hfperclkdiv: u8,
    /// Encoding of Peripheral Clock divisor
    pub hfperclkdivcode: u8,
    /// Encoding of Core Clock divisor
    pub hfcoreclkdivcode: u8,
    /// 2 or 4
    pub hfcoreclklediv: u8,
    /// HFCLK/hclkdiv
    pub hclkfreq: u32,
    /// HFCLK/hclkdiv/corediv
    pub hfcoreclkfreq: u32,
    /// HFCLK/hclkdiv/perdiv
    pub hfperclkfreq: u32,
}

/// Get the device information data.
fn dev_info() -> &'static RegisterBlock {
    unsafe { &*pac::DEVINFO::ptr() }
}

/// Get the Production Revision of the chip
fn get_prod_rev() -> u8 {
    let di = dev_info();
    di.part.read().prod_rev().bits()
}

/// Returns the clock source, the basefreq, the divisor for HCLK,
/// the HCLK frequency, the core frequency, the divisor for the core
/// frequency, the Peripheral Clock Frequency and the divisor used for
/// peripheral frequency.
pub fn get_clock_config() -> ClockConfiguration {
    let mut basefreq: u32 = DEFAULT_HFRCO_FREQUENCY.0;
    let mut source = ClockSource::None;
    let cmu = unsafe { &*pac::CMU::ptr() };
    let status = cmu.status.read();

    if status.hfrcosel().bit() {
        if let Some(band) = cmu.hfrcoctrl.read().band().variant() {
            match band {
                hfrcoctrl::BAND_A::_1MHZ => {
                    if get_prod_rev() >= 19 {
                        basefreq = 1200000;
                    } else {
                        basefreq = 1000000;
                    }
                }
                hfrcoctrl::BAND_A::_7MHZ => {
                    if get_prod_rev() >= 19 {
                        basefreq = 6600000;
                    } else {
                        basefreq = 7000000;
                    }
                }
                hfrcoctrl::BAND_A::_11MHZ => {
                    basefreq = 11000000;
                }
                hfrcoctrl::BAND_A::_14MHZ => {
                    basefreq = 14000000;
                }
                hfrcoctrl::BAND_A::_21MHZ => {
                    basefreq = 21000000;
                }
            }
            source = ClockSource::HFRCO(Hertz(basefreq));
        }
    } else if status.lfrcosel().bit() {
        basefreq = DEFAULT_LFRCO_FREQUENCY.0;
        source = ClockSource::LFRCO(Hertz(basefreq));
    } else if status.lfxosel().bit() {
        basefreq = LFXO_FREQUENCY.0;
        source = ClockSource::LFXO(Hertz(basefreq));
    } else if status.hfxosel().bit() {
        basefreq = HFXO_FREQUENCY.0;
        source = ClockSource::HFXO(Hertz(basefreq));
    } else {
        source = ClockSource::None;
    }

    let hclkdiv = cmu.ctrl.read().hfclkdiv().bits();
    let corediv = cmu.hfcoreclkdiv.read().hfcoreclkdiv().bits();
    let hfcoreclklediv = 1 << (cmu.hfcoreclkdiv.read().hfcoreclklediv().bit() as u8 + 1);
    let perdiv = cmu.hfperclkdiv.read().hfperclkdiv().bits();
    let hclkfreq = basefreq / (hclkdiv as u32 + 1);
    let corefreq = hclkfreq / (1 << corediv);
    let perfreq = hclkfreq / (1 << perdiv);

    ClockConfiguration {
        source,
        basefreq,
        hclkdiv: hclkdiv + 1,
        hclkfreq,
        hfcoreclkfreq: corefreq,
        hfcoreclkdiv: 1 << corediv,
        hfcoreclkdivcode: corediv,
        hfperclkfreq: perfreq,
        hfperclkdiv: 1 << perdiv,
        hfperclkdivcode: perdiv,
        hfcoreclklediv,
    }
}

/// Configures wait states for the system while the clock frequency is being changed.
fn prepare_clock_change<T: FnOnce() -> ()>(freq: u32, wait_clock_ready: T) {
    let msc = unsafe { &*pac::MSC::ptr() };
    let msc_read_ctrl = &msc.readctrl;

    // MSC_READCTL:
    //
    // If software wants to set a core clock frequency above 16 MHz, this register
    // must be set to WS1 before the core clock is switched to the higher frequency.
    // When changing to a lower frequency, this register can be set to WS0 after the
    // frequency transition has been completed. After reset, the core clock is 14 MHz
    // from the HFRCO but the MODE field of MSC_READCTRL register is set to WS1. This
    // is because the HFRCO may produce a frequency above 16 MHz before it is calibrated.
    // If the HFRCO is used as clock source, wait until the oscillator is stable on
    // the new frequency to avoid unpredictable behavior.
    //
    // Value | Mode | Description
    // ------|------|-------------------------------------------------------------------
    // 0     | WS0  | Zero wait-states inserted in fetch or read transfers.
    // 1     | WS1  | One wait-state inserted for each fetch or read transfer. This mode
    //       |      | is required for a core frequency above 16 MHz

    msc_read_ctrl.write(|w| w.mode().variant(MODE_A::WS1));
    wait_clock_ready();
    if freq <= WAIT_STATE_1_THRESHOLD {
        msc_read_ctrl.write(|w| w.mode().variant(MODE_A::WS0));
    }
}
