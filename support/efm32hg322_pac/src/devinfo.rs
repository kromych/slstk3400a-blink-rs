#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cal: Cal,
    adc0cal0: Adc0cal0,
    adc0cal1: Adc0cal1,
    adc0cal2: Adc0cal2,
    _reserved4: [u8; 0x08],
    idac0cal0: Idac0cal0,
    ushfrcocal0: Ushfrcocal0,
    _reserved6: [u8; 0x04],
    auxhfrcocal0: Auxhfrcocal0,
    auxhfrcocal1: Auxhfrcocal1,
    _reserved8: [u8; 0x03],
    hfrcocal0: Hfrcocal0,
    hfrcocal1: Hfrcocal1,
    _reserved10: [u8; 0x0f],
    uniquel: Uniquel,
    uniqueh: Uniqueh,
    msize: Msize,
    part: Part,
}
impl RegisterBlock {
    #[doc = "0x00 - Calibration temperature and checksum"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    #[doc = "0x04 - ADC0 Calibration register 0"]
    #[inline(always)]
    pub const fn adc0cal0(&self) -> &Adc0cal0 {
        &self.adc0cal0
    }
    #[doc = "0x08 - ADC0 Calibration register 1"]
    #[inline(always)]
    pub const fn adc0cal1(&self) -> &Adc0cal1 {
        &self.adc0cal1
    }
    #[doc = "0x0c - ADC0 Calibration register 2"]
    #[inline(always)]
    pub const fn adc0cal2(&self) -> &Adc0cal2 {
        &self.adc0cal2
    }
    #[doc = "0x18 - IDAC0 calibration register"]
    #[inline(always)]
    pub const fn idac0cal0(&self) -> &Idac0cal0 {
        &self.idac0cal0
    }
    #[doc = "0x1c - USHFRCO calibration register"]
    #[inline(always)]
    pub const fn ushfrcocal0(&self) -> &Ushfrcocal0 {
        &self.ushfrcocal0
    }
    #[doc = "0x24 - AUXHFRCO calibration register 0"]
    #[inline(always)]
    pub const fn auxhfrcocal0(&self) -> &Auxhfrcocal0 {
        &self.auxhfrcocal0
    }
    #[doc = "0x28 - AUXHFRCO calibration register 1"]
    #[inline(always)]
    pub const fn auxhfrcocal1(&self) -> &Auxhfrcocal1 {
        &self.auxhfrcocal1
    }
    #[doc = "0x2c - HFRCO calibration register 0"]
    #[inline(always)]
    pub const fn hfrcocal0(&self) -> &Hfrcocal0 {
        &self.hfrcocal0
    }
    #[doc = "0x30 - HFRCO calibration register 1"]
    #[inline(always)]
    pub const fn hfrcocal1(&self) -> &Hfrcocal1 {
        &self.hfrcocal1
    }
    #[doc = "0x40 - Low 32 bits of device unique number"]
    #[inline(always)]
    pub const fn uniquel(&self) -> &Uniquel {
        &self.uniquel
    }
    #[doc = "0x44 - High 32 bits of device unique number"]
    #[inline(always)]
    pub const fn uniqueh(&self) -> &Uniqueh {
        &self.uniqueh
    }
    #[doc = "0x48 - Flash and SRAM Memory size in KiloBytes"]
    #[inline(always)]
    pub const fn msize(&self) -> &Msize {
        &self.msize
    }
    #[doc = "0x4c - Part description"]
    #[inline(always)]
    pub const fn part(&self) -> &Part {
        &self.part
    }
}
#[doc = "CAL (r) register accessor: Calibration temperature and checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "Calibration temperature and checksum"]
pub mod cal;
#[doc = "ADC0CAL0 (r) register accessor: ADC0 Calibration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0cal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0cal0`] module"]
#[doc(alias = "ADC0CAL0")]
pub type Adc0cal0 = crate::Reg<adc0cal0::Adc0cal0Spec>;
#[doc = "ADC0 Calibration register 0"]
pub mod adc0cal0;
#[doc = "ADC0CAL1 (r) register accessor: ADC0 Calibration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0cal1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0cal1`] module"]
#[doc(alias = "ADC0CAL1")]
pub type Adc0cal1 = crate::Reg<adc0cal1::Adc0cal1Spec>;
#[doc = "ADC0 Calibration register 1"]
pub mod adc0cal1;
#[doc = "ADC0CAL2 (r) register accessor: ADC0 Calibration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0cal2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0cal2`] module"]
#[doc(alias = "ADC0CAL2")]
pub type Adc0cal2 = crate::Reg<adc0cal2::Adc0cal2Spec>;
#[doc = "ADC0 Calibration register 2"]
pub mod adc0cal2;
#[doc = "IDAC0CAL0 (r) register accessor: IDAC0 calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`idac0cal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idac0cal0`] module"]
#[doc(alias = "IDAC0CAL0")]
pub type Idac0cal0 = crate::Reg<idac0cal0::Idac0cal0Spec>;
#[doc = "IDAC0 calibration register"]
pub mod idac0cal0;
#[doc = "USHFRCOCAL0 (r) register accessor: USHFRCO calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcocal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ushfrcocal0`] module"]
#[doc(alias = "USHFRCOCAL0")]
pub type Ushfrcocal0 = crate::Reg<ushfrcocal0::Ushfrcocal0Spec>;
#[doc = "USHFRCO calibration register"]
pub mod ushfrcocal0;
#[doc = "AUXHFRCOCAL0 (r) register accessor: AUXHFRCO calibration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`auxhfrcocal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxhfrcocal0`] module"]
#[doc(alias = "AUXHFRCOCAL0")]
pub type Auxhfrcocal0 = crate::Reg<auxhfrcocal0::Auxhfrcocal0Spec>;
#[doc = "AUXHFRCO calibration register 0"]
pub mod auxhfrcocal0;
#[doc = "AUXHFRCOCAL1 (r) register accessor: AUXHFRCO calibration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`auxhfrcocal1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxhfrcocal1`] module"]
#[doc(alias = "AUXHFRCOCAL1")]
pub type Auxhfrcocal1 = crate::Reg<auxhfrcocal1::Auxhfrcocal1Spec>;
#[doc = "AUXHFRCO calibration register 1"]
pub mod auxhfrcocal1;
#[doc = "HFRCOCAL0 (r) register accessor: HFRCO calibration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcocal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcocal0`] module"]
#[doc(alias = "HFRCOCAL0")]
pub type Hfrcocal0 = crate::Reg<hfrcocal0::Hfrcocal0Spec>;
#[doc = "HFRCO calibration register 0"]
pub mod hfrcocal0;
#[doc = "HFRCOCAL1 (r) register accessor: HFRCO calibration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcocal1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcocal1`] module"]
#[doc(alias = "HFRCOCAL1")]
pub type Hfrcocal1 = crate::Reg<hfrcocal1::Hfrcocal1Spec>;
#[doc = "HFRCO calibration register 1"]
pub mod hfrcocal1;
#[doc = "UNIQUEL (r) register accessor: Low 32 bits of device unique number\n\nYou can [`read`](crate::Reg::read) this register and get [`uniquel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uniquel`] module"]
#[doc(alias = "UNIQUEL")]
pub type Uniquel = crate::Reg<uniquel::UniquelSpec>;
#[doc = "Low 32 bits of device unique number"]
pub mod uniquel;
#[doc = "UNIQUEH (r) register accessor: High 32 bits of device unique number\n\nYou can [`read`](crate::Reg::read) this register and get [`uniqueh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uniqueh`] module"]
#[doc(alias = "UNIQUEH")]
pub type Uniqueh = crate::Reg<uniqueh::UniquehSpec>;
#[doc = "High 32 bits of device unique number"]
pub mod uniqueh;
#[doc = "MSIZE (r) register accessor: Flash and SRAM Memory size in KiloBytes\n\nYou can [`read`](crate::Reg::read) this register and get [`msize::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msize`] module"]
#[doc(alias = "MSIZE")]
pub type Msize = crate::Reg<msize::MsizeSpec>;
#[doc = "Flash and SRAM Memory size in KiloBytes"]
pub mod msize;
#[doc = "PART (r) register accessor: Part description\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@part`] module"]
#[doc(alias = "PART")]
pub type Part = crate::Reg<part::PartSpec>;
#[doc = "Part description"]
pub mod part;
