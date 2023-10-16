use crate::time_util::Hertz;
use crate::HfrcoBand;
use defmt::Format;
use efm32hg322_pac as pac;
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
#[derive(Clone, Copy)]
pub enum ClockSource {
    /// Low Frequency Crystal Oscillator
    LFXO,
    /// Low Frequency Internal RC Oscillator
    LFRCO,
    /// High Frequency Internal RC Oscillator
    HFRCO(HfrcoBand),
    /// High Frequency Crystall Oscillator
    HFXO,
    /// Ultra Low Frequency
    ULFRCO,
    /// HF Core Clock divided by 2
    HFCoreClock2,
}

impl Format for ClockSource {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            ClockSource::LFXO => defmt::write!(fmt, "LFXO"),
            ClockSource::LFRCO => defmt::write!(fmt, "LFRCO"),
            ClockSource::HFRCO(band) => match band {
                HfrcoBand::_1MHZ => defmt::write!(fmt, "HFRCO 1MHz"),
                HfrcoBand::_7MHZ => defmt::write!(fmt, "HFRCO 7MHz"),
                HfrcoBand::_11MHZ => defmt::write!(fmt, "HFRCO 11MHz"),
                HfrcoBand::_14MHZ => defmt::write!(fmt, "HFRCO 14MHz"),
                HfrcoBand::_21MHZ => defmt::write!(fmt, "HFRCO 21MHz"),
            },
            ClockSource::HFXO => defmt::write!(fmt, "HFXO"),
            ClockSource::ULFRCO => defmt::write!(fmt, "ULFRCO"),
            ClockSource::HFCoreClock2 => defmt::write!(fmt, "HFCoreClock2"),
        }
    }
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

impl Default for ClockConfiguration {
    fn default() -> Self {
        ClockConfiguration {
            source: ClockSource::HFRCO(HfrcoBand::_14MHZ),
            basefreq: DEFAULT_HFRCO_FREQUENCY.0,
            hclkdiv: 1,
            hfcoreclkdiv: 1,
            hfperclkdiv: 1,
            hfperclkdivcode: 0,
            hfcoreclkdivcode: 0,
            hfcoreclklediv: 2,
            hclkfreq: DEFAULT_HFRCO_FREQUENCY.0,
            hfcoreclkfreq: DEFAULT_HFRCO_FREQUENCY.0,
            hfperclkfreq: DEFAULT_HFRCO_FREQUENCY.0,
        }
    }
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
pub fn get_clock_config() -> Result<ClockConfiguration, ()> {
    let source;
    let basefreq;
    let cmu = unsafe { &*pac::CMU::ptr() };
    let status = cmu.status.read();

    if status.hfrcosel().bit() {
        if let Some(band) = cmu.hfrcoctrl.read().band().variant() {
            match band {
                HfrcoBand::_1MHZ => {
                    if get_prod_rev() >= 19 {
                        basefreq = 1200000;
                    } else {
                        basefreq = 1000000;
                    }
                }
                HfrcoBand::_7MHZ => {
                    if get_prod_rev() >= 19 {
                        basefreq = 6600000;
                    } else {
                        basefreq = 7000000;
                    }
                }
                HfrcoBand::_11MHZ => {
                    basefreq = 11000000;
                }
                HfrcoBand::_14MHZ => {
                    basefreq = 14000000;
                }
                HfrcoBand::_21MHZ => {
                    basefreq = 21000000;
                }
            }
            source = ClockSource::HFRCO(band);
        } else {
            return Err(());
        }
    } else if status.lfrcosel().bit() {
        basefreq = DEFAULT_LFRCO_FREQUENCY.0;
        source = ClockSource::LFRCO;
    } else if status.lfxosel().bit() {
        basefreq = LFXO_FREQUENCY.0;
        source = ClockSource::LFXO;
    } else if status.hfxosel().bit() {
        basefreq = HFXO_FREQUENCY.0;
        source = ClockSource::HFXO;
    } else {
        return Err(());
    }

    let hclkdiv = cmu.ctrl.read().hfclkdiv().bits();
    let corediv = cmu.hfcoreclkdiv.read().hfcoreclkdiv().bits();
    let hfcoreclklediv = 1 << (cmu.hfcoreclkdiv.read().hfcoreclklediv().bit() as u8 + 1);
    let perdiv = cmu.hfperclkdiv.read().hfperclkdiv().bits();
    let hclkfreq = basefreq / (hclkdiv as u32 + 1);
    let corefreq = hclkfreq / (1 << corediv);
    let perfreq = hclkfreq / (1 << perdiv);

    Ok(ClockConfiguration {
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
    })
}

pub fn set_clock_config(clock_config: ClockConfiguration) -> Result<ClockConfiguration, ()> {
    // Set wait states for the worst case for the flash access time.
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

    let change_freq_and_wait = || {
        let cmu = unsafe { &*pac::CMU::ptr() };

        match clock_config.source {
            ClockSource::HFRCO(band) => {
                // Configure band and tuning.
                let di = dev_info();
                let tuning = match band {
                    HfrcoBand::_1MHZ => di.hfrcocal0.read().band1().bits(),
                    HfrcoBand::_7MHZ => di.hfrcocal0.read().band7().bits(),
                    HfrcoBand::_11MHZ => di.hfrcocal0.read().band11().bits(),
                    HfrcoBand::_14MHZ => di.hfrcocal0.read().band14().bits(),
                    HfrcoBand::_21MHZ => di.hfrcocal1.read().band21().bits(),
                };
                cmu.hfrcoctrl.write(|w| {
                    w.band().variant(band);
                    w.tuning().variant(tuning)
                });

                // Check if HFRCO is already enabled.
                if !cmu.status.read().hfrcordy().bit() {
                    // Enable HFRCO and wait until it is stable.
                    cmu.oscencmd.write(|w| w.hfrcoen().set_bit());
                    // Wait until ready.
                    while !cmu.status.read().hfrcoens().bit() {}
                    while !cmu.status.read().hfrcordy().bit() {}
                }

                // Select HFRCO as source for HFCLK.
                cmu.cmd.write(|w| w.hfclksel().hfrco());
                while !cmu.status.read().hfrcosel().bit() {}
            }
            ClockSource::HFXO => {
                // Check if HFXO is already enabled.
                if !cmu.status.read().hfxordy().bit() {
                    // Enable HFXO and wait until it is stable.
                    cmu.oscencmd.write(|w| w.hfxoen().set_bit());
                    while !cmu.status.read().hfxoens().bit() {}
                    while !cmu.status.read().hfxordy().bit() {}
                }

                // Select HFXO as source for HFCLK.
                cmu.cmd.write(|w| w.hfclksel().hfxo());
                while !cmu.status.read().hfxosel().bit() {}
            }
            ClockSource::LFRCO => {
                // Check if LFRCO is already enabled.
                if !cmu.status.read().lfrcordy().bit() {
                    // Enable LFRCO and wait until it is stable.
                    cmu.oscencmd.write(|w| w.lfrcoen().set_bit());
                    while !cmu.status.read().lfrcoens().bit() {}
                    while !cmu.status.read().lfrcordy().bit() {}
                }

                // Select LFRCO as source for HFCLK.
                cmu.cmd.write(|w| w.hfclksel().lfrco());
                while !cmu.status.read().lfrcosel().bit() {}
            }
            ClockSource::LFXO => {
                // Check if LFXO is already enabled.
                if !cmu.status.read().lfxordy().bit() {
                    // Enable LFXO and wait until it is stable.
                    cmu.oscencmd.write(|w| w.lfxoen().set_bit());
                    while !cmu.status.read().lfxoens().bit() {}
                    while !cmu.status.read().lfxordy().bit() {}
                }

                // Select LFXO as source for HFCLK.
                cmu.cmd.write(|w| w.hfclksel().lfxo());
                while !cmu.status.read().lfxosel().bit() {}
            }
            ClockSource::ULFRCO => Err(())?,
            ClockSource::HFCoreClock2 => Err(())?,
        }

        Ok(())
    };

    // TODO: restore wait states?
    change_freq_and_wait()?;

    // TODO: assert on the configuration matching what was intended to have been set.
    let clock_config = get_clock_config()?;

    // If the new frequency is below the threshold, no wait state is rwquired when reading the flash.
    if clock_config.basefreq <= WAIT_STATE_1_THRESHOLD {
        msc_read_ctrl.write(|w| w.mode().variant(MODE_A::WS0));
    }

    Ok(clock_config)
}

pub fn lock_clock_config() {
    todo!("lock_clock_config");
}
