#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `SINGLEACT` reader - Single Conversion Active"]
pub type SINGLEACT_R = crate::BitReader;
#[doc = "Field `SCANACT` reader - Scan Conversion Active"]
pub type SCANACT_R = crate::BitReader;
#[doc = "Field `SINGLEREFWARM` reader - Single Reference Warmed Up"]
pub type SINGLEREFWARM_R = crate::BitReader;
#[doc = "Field `SCANREFWARM` reader - Scan Reference Warmed Up"]
pub type SCANREFWARM_R = crate::BitReader;
#[doc = "Field `WARM` reader - ADC Warmed Up"]
pub type WARM_R = crate::BitReader;
#[doc = "Field `SINGLEDV` reader - Single Sample Data Valid"]
pub type SINGLEDV_R = crate::BitReader;
#[doc = "Field `SCANDV` reader - Scan Data Valid"]
pub type SCANDV_R = crate::BitReader;
#[doc = "Field `SCANDATASRC` reader - Scan Data Source"]
pub type SCANDATASRC_R = crate::FieldReader<SCANDATASRC_A>;
#[doc = "Scan Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCANDATASRC_A {
    #[doc = "0: Single ended mode: SCANDATA result originates from ADCn_CH0. Differential mode: SCANDATA result originates from ADCn_CH0-ADCn_CH1"]
    CH0 = 0,
    #[doc = "1: Single ended mode: SCANDATA result originates from ADCn_CH1. Differential mode: SCANDATA result originates from ADCn_CH2_ADCn_CH3"]
    CH1 = 1,
    #[doc = "2: Single ended mode: SCANDATA result originates from ADCn_CH2. Differential mode: SCANDATA result originates from ADCn_CH4-ADCn_CH5"]
    CH2 = 2,
    #[doc = "3: Single ended mode: SCANDATA result originates from ADCn_CH3. Differential mode: SCANDATA result originates from ADCn_CH6-ADCn_CH7"]
    CH3 = 3,
    #[doc = "4: SCANDATA result originates from ADCn_CH4"]
    CH4 = 4,
    #[doc = "5: SCANDATA result originates from ADCn_CH5"]
    CH5 = 5,
    #[doc = "6: SCANDATA result originates from ADCn_CH6"]
    CH6 = 6,
    #[doc = "7: SCANDATA result originates from ADCn_CH7"]
    CH7 = 7,
}
impl From<SCANDATASRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANDATASRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCANDATASRC_A {
    type Ux = u8;
}
impl SCANDATASRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCANDATASRC_A {
        match self.bits {
            0 => SCANDATASRC_A::CH0,
            1 => SCANDATASRC_A::CH1,
            2 => SCANDATASRC_A::CH2,
            3 => SCANDATASRC_A::CH3,
            4 => SCANDATASRC_A::CH4,
            5 => SCANDATASRC_A::CH5,
            6 => SCANDATASRC_A::CH6,
            7 => SCANDATASRC_A::CH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH0. Differential mode: SCANDATA result originates from ADCn_CH0-ADCn_CH1"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == SCANDATASRC_A::CH0
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH1. Differential mode: SCANDATA result originates from ADCn_CH2_ADCn_CH3"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == SCANDATASRC_A::CH1
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH2. Differential mode: SCANDATA result originates from ADCn_CH4-ADCn_CH5"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == SCANDATASRC_A::CH2
    }
    #[doc = "Single ended mode: SCANDATA result originates from ADCn_CH3. Differential mode: SCANDATA result originates from ADCn_CH6-ADCn_CH7"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == SCANDATASRC_A::CH3
    }
    #[doc = "SCANDATA result originates from ADCn_CH4"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == SCANDATASRC_A::CH4
    }
    #[doc = "SCANDATA result originates from ADCn_CH5"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == SCANDATASRC_A::CH5
    }
    #[doc = "SCANDATA result originates from ADCn_CH6"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == SCANDATASRC_A::CH6
    }
    #[doc = "SCANDATA result originates from ADCn_CH7"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == SCANDATASRC_A::CH7
    }
}
impl R {
    #[doc = "Bit 0 - Single Conversion Active"]
    #[inline(always)]
    pub fn singleact(&self) -> SINGLEACT_R {
        SINGLEACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Active"]
    #[inline(always)]
    pub fn scanact(&self) -> SCANACT_R {
        SCANACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Reference Warmed Up"]
    #[inline(always)]
    pub fn singlerefwarm(&self) -> SINGLEREFWARM_R {
        SINGLEREFWARM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Reference Warmed Up"]
    #[inline(always)]
    pub fn scanrefwarm(&self) -> SCANREFWARM_R {
        SCANREFWARM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC Warmed Up"]
    #[inline(always)]
    pub fn warm(&self) -> WARM_R {
        WARM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Sample Data Valid"]
    #[inline(always)]
    pub fn singledv(&self) -> SINGLEDV_R {
        SINGLEDV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Data Valid"]
    #[inline(always)]
    pub fn scandv(&self) -> SCANDV_R {
        SCANDV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Scan Data Source"]
    #[inline(always)]
    pub fn scandatasrc(&self) -> SCANDATASRC_R {
        SCANDATASRC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
