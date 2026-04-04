#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `SINGLEACT` reader - Single Conversion Active"]
pub type SingleactR = crate::BitReader;
#[doc = "Field `SCANACT` reader - Scan Conversion Active"]
pub type ScanactR = crate::BitReader;
#[doc = "Field `SINGLEREFWARM` reader - Single Reference Warmed Up"]
pub type SinglerefwarmR = crate::BitReader;
#[doc = "Field `SCANREFWARM` reader - Scan Reference Warmed Up"]
pub type ScanrefwarmR = crate::BitReader;
#[doc = "Field `WARM` reader - ADC Warmed Up"]
pub type WarmR = crate::BitReader;
#[doc = "Field `SINGLEDV` reader - Single Sample Data Valid"]
pub type SingledvR = crate::BitReader;
#[doc = "Field `SCANDV` reader - Scan Data Valid"]
pub type ScandvR = crate::BitReader;
#[doc = "Scan Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scandatasrc {
    #[doc = "0: Single ended mode: SCANDATA result originates from ADCn_CH0. Differential mode: SCANDATA result originates from ADCn_CH0-ADCn_CH1"]
    Ch0 = 0,
    #[doc = "1: Single ended mode: SCANDATA result originates from ADCn_CH1. Differential mode: SCANDATA result originates from ADCn_CH2_ADCn_CH3"]
    Ch1 = 1,
    #[doc = "2: Single ended mode: SCANDATA result originates from ADCn_CH2. Differential mode: SCANDATA result originates from ADCn_CH4-ADCn_CH5"]
    Ch2 = 2,
    #[doc = "3: Single ended mode: SCANDATA result originates from ADCn_CH3. Differential mode: SCANDATA result originates from ADCn_CH6-ADCn_CH7"]
    Ch3 = 3,
    #[doc = "4: SCANDATA result originates from ADCn_CH4"]
    Ch4 = 4,
    #[doc = "5: SCANDATA result originates from ADCn_CH5"]
    Ch5 = 5,
    #[doc = "6: SCANDATA result originates from ADCn_CH6"]
    Ch6 = 6,
    #[doc = "7: SCANDATA result originates from ADCn_CH7"]
    Ch7 = 7,
}
impl From<Scandatasrc> for u8 {
    #[inline(always)]
    fn from(variant: Scandatasrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scandatasrc {
    type Ux = u8;
}
impl crate::IsEnum for Scandatasrc {}
#[doc = "Field `SCANDATASRC` reader - Scan Data Source"]
pub type ScandatasrcR = crate::FieldReader<Scandatasrc>;
impl ScandatasrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scandatasrc {
        match self.bits {
            0 => Scandatasrc::Ch0,
            1 => Scandatasrc::Ch1,
            2 => Scandatasrc::Ch2,
            3 => Scandatasrc::Ch3,
            4 => Scandatasrc::Ch4,
            5 => Scandatasrc::Ch5,
            6 => Scandatasrc::Ch6,
            7 => Scandatasrc::Ch7,
            _ => unreachable!(),
        }
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH0. Differential mode: SCANDATA result originates from ADCn_CH0-ADCn_CH1"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == Scandatasrc::Ch0
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH1. Differential mode: SCANDATA result originates from ADCn_CH2_ADCn_CH3"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == Scandatasrc::Ch1
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH2. Differential mode: SCANDATA result originates from ADCn_CH4-ADCn_CH5"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == Scandatasrc::Ch2
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH3. Differential mode: SCANDATA result originates from ADCn_CH6-ADCn_CH7"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == Scandatasrc::Ch3
    }
    #[doc = "SCANDATA result originates from ADCn_CH4"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == Scandatasrc::Ch4
    }
    #[doc = "SCANDATA result originates from ADCn_CH5"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == Scandatasrc::Ch5
    }
    #[doc = "SCANDATA result originates from ADCn_CH6"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == Scandatasrc::Ch6
    }
    #[doc = "SCANDATA result originates from ADCn_CH7"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == Scandatasrc::Ch7
    }
}
impl R {
    #[doc = "Bit 0 - Single Conversion Active"]
    #[inline(always)]
    pub fn singleact(&self) -> SingleactR {
        SingleactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Active"]
    #[inline(always)]
    pub fn scanact(&self) -> ScanactR {
        ScanactR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Reference Warmed Up"]
    #[inline(always)]
    pub fn singlerefwarm(&self) -> SinglerefwarmR {
        SinglerefwarmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Reference Warmed Up"]
    #[inline(always)]
    pub fn scanrefwarm(&self) -> ScanrefwarmR {
        ScanrefwarmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC Warmed Up"]
    #[inline(always)]
    pub fn warm(&self) -> WarmR {
        WarmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Sample Data Valid"]
    #[inline(always)]
    pub fn singledv(&self) -> SingledvR {
        SingledvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Data Valid"]
    #[inline(always)]
    pub fn scandv(&self) -> ScandvR {
        ScandvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Scan Data Source"]
    #[inline(always)]
    pub fn scandatasrc(&self) -> ScandatasrcR {
        ScandatasrcR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
