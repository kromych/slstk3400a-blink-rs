#[doc = "Register `HFRCOCAL1` reader"]
pub type R = crate::R<HFRCOCAL1_SPEC>;
#[doc = "Field `BAND21` reader - 21MHz tuning value for HFRCO"]
pub type BAND21_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 21MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band21(&self) -> BAND21_R {
        BAND21_R::new(self.bits)
    }
}
#[doc = "HFRCO calibration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfrcocal1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFRCOCAL1_SPEC;
impl crate::RegisterSpec for HFRCOCAL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfrcocal1::R`](R) reader structure"]
impl crate::Readable for HFRCOCAL1_SPEC {}
