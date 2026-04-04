#[doc = "Register `ADC0CAL2` reader"]
pub type R = crate::R<Adc0cal2Spec>;
#[doc = "Field `2XVDDVSS_OFFSET` reader - Offset for 2XVDDVSS reference"]
pub type _2xvddvssOffsetR = crate::FieldReader;
#[doc = "Field `TEMP1V25` reader - Temperature reading at 1V25 reference"]
pub type Temp1v25R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:6 - Offset for 2XVDDVSS reference"]
    #[inline(always)]
    pub fn _2xvddvss_offset(&self) -> _2xvddvssOffsetR {
        _2xvddvssOffsetR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 20:31 - Temperature reading at 1V25 reference"]
    #[inline(always)]
    pub fn temp1v25(&self) -> Temp1v25R {
        Temp1v25R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "ADC0 Calibration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0cal2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0cal2Spec;
impl crate::RegisterSpec for Adc0cal2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0cal2::R`](R) reader structure"]
impl crate::Readable for Adc0cal2Spec {}
