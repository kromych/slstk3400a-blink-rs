#[doc = "Register `ADC0CAL0` reader"]
pub type R = crate::R<Adc0cal0Spec>;
#[doc = "Field `1V25_OFFSET` reader - Offset for 1V25 reference"]
pub type _1v25OffsetR = crate::FieldReader;
#[doc = "Field `1V25_GAIN` reader - Gain for 1V25 reference"]
pub type _1v25GainR = crate::FieldReader;
#[doc = "Field `2V5_OFFSET` reader - Offset for 2V5 reference"]
pub type _2v5OffsetR = crate::FieldReader;
#[doc = "Field `2V5_GAIN` reader - Gain for 2V5 reference"]
pub type _2v5GainR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Offset for 1V25 reference"]
    #[inline(always)]
    pub fn _1v25_offset(&self) -> _1v25OffsetR {
        _1v25OffsetR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for 1V25 reference"]
    #[inline(always)]
    pub fn _1v25_gain(&self) -> _1v25GainR {
        _1v25GainR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for 2V5 reference"]
    #[inline(always)]
    pub fn _2v5_offset(&self) -> _2v5OffsetR {
        _2v5OffsetR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for 2V5 reference"]
    #[inline(always)]
    pub fn _2v5_gain(&self) -> _2v5GainR {
        _2v5GainR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "ADC0 Calibration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0cal0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0cal0Spec;
impl crate::RegisterSpec for Adc0cal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0cal0::R`](R) reader structure"]
impl crate::Readable for Adc0cal0Spec {}
