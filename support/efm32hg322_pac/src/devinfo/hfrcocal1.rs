#[doc = "Register `HFRCOCAL1` reader"]
pub type R = crate::R<Hfrcocal1Spec>;
#[doc = "Field `BAND21` reader - 21MHz tuning value for HFRCO"]
pub type Band21R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 21MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band21(&self) -> Band21R {
        Band21R::new(self.bits)
    }
}
#[doc = "HFRCO calibration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcocal1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfrcocal1Spec;
impl crate::RegisterSpec for Hfrcocal1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfrcocal1::R`](R) reader structure"]
impl crate::Readable for Hfrcocal1Spec {}
