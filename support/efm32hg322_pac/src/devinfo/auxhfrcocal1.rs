#[doc = "Register `AUXHFRCOCAL1` reader"]
pub type R = crate::R<Auxhfrcocal1Spec>;
#[doc = "Field `BAND21` reader - 21MHz tuning value for AUXHFRCO"]
pub type Band21R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 21MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band21(&self) -> Band21R {
        Band21R::new(self.bits)
    }
}
#[doc = "AUXHFRCO calibration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`auxhfrcocal1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Auxhfrcocal1Spec;
impl crate::RegisterSpec for Auxhfrcocal1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`auxhfrcocal1::R`](R) reader structure"]
impl crate::Readable for Auxhfrcocal1Spec {}
