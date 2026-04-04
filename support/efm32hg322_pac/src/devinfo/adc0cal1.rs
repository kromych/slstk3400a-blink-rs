#[doc = "Register `ADC0CAL1` reader"]
pub type R = crate::R<Adc0cal1Spec>;
#[doc = "Field `VDD_OFFSET` reader - Offset for VDD reference"]
pub type VddOffsetR = crate::FieldReader;
#[doc = "Field `VDD_GAIN` reader - Gain for VDD reference"]
pub type VddGainR = crate::FieldReader;
#[doc = "Field `5VDIFF_OFFSET` reader - Offset for 5VDIFF reference"]
pub type _5vdiffOffsetR = crate::FieldReader;
#[doc = "Field `5VDIFF_GAIN` reader - Gain for 5VDIFF reference"]
pub type _5vdiffGainR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Offset for VDD reference"]
    #[inline(always)]
    pub fn vdd_offset(&self) -> VddOffsetR {
        VddOffsetR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for VDD reference"]
    #[inline(always)]
    pub fn vdd_gain(&self) -> VddGainR {
        VddGainR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for 5VDIFF reference"]
    #[inline(always)]
    pub fn _5vdiff_offset(&self) -> _5vdiffOffsetR {
        _5vdiffOffsetR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for 5VDIFF reference"]
    #[inline(always)]
    pub fn _5vdiff_gain(&self) -> _5vdiffGainR {
        _5vdiffGainR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "ADC0 Calibration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0cal1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0cal1Spec;
impl crate::RegisterSpec for Adc0cal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0cal1::R`](R) reader structure"]
impl crate::Readable for Adc0cal1Spec {}
