//! Time helpers
//!
//! These are copied from stm32f30x-hal's time helpers (from
//! <https://github.com/japaric/stm32f30x-hal/blob/master/src/time.rs>) on demand.

/// Bits per second
#[derive(Clone, Copy)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Clone, Copy)]
pub struct Hertz(pub u32);

/// KiloHertz
#[derive(Clone, Copy)]
pub struct KiloHertz(pub u32);

/// MegaHertz
#[derive(Clone, Copy)]
pub struct MegaHertz(pub u32);

#[derive(Clone, Copy)]
pub struct Seconds(u32);

#[derive(Clone, Copy)]
pub struct MilliSeconds(u32);

#[derive(Clone, Copy)]
pub struct MicroSeconds(u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `MicroSeconds`
    fn us(self) -> MicroSeconds;

    /// Wrap in `MilliSeconds`
    fn ms(self) -> MilliSeconds;

    /// Wrap in `Seconds`
    fn s(self) -> Seconds;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    fn us(self) -> MicroSeconds {
        MicroSeconds(self)
    }

    fn ms(self) -> MilliSeconds {
        MilliSeconds(self)
    }

    fn s(self) -> Seconds {
        Seconds(self)
    }
}

impl From<KiloHertz> for Hertz {
    fn from(val: KiloHertz) -> Self {
        Hertz(val.0 * 1_000)
    }
}

impl From<MegaHertz> for Hertz {
    fn from(val: MegaHertz) -> Self {
        Hertz(val.0 * 1_000_000)
    }
}

impl From<MegaHertz> for KiloHertz {
    fn from(val: MegaHertz) -> Self {
        KiloHertz(val.0 * 1_000)
    }
}

impl From<Seconds> for MilliSeconds {
    fn from(val: Seconds) -> Self {
        MilliSeconds(val.0 * 1000)
    }
}

impl From<Seconds> for MicroSeconds {
    fn from(val: Seconds) -> Self {
        MicroSeconds(val.0 * 1_000_000)
    }
}

impl From<MilliSeconds> for MicroSeconds {
    fn from(val: MilliSeconds) -> Self {
        MicroSeconds(val.0 * 1000)
    }
}
