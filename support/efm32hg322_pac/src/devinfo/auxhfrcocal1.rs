#[doc = "Register `AUXHFRCOCAL1` reader"]
pub type R = crate::R<AUXHFRCOCAL1_SPEC>;
#[doc = "Field `BAND21` reader - 21MHz tuning value for AUXHFRCO"]
pub type BAND21_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 21MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band21(&self) -> BAND21_R {
        BAND21_R::new(self.bits)
    }
}
#[doc = "AUXHFRCO calibration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxhfrcocal1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUXHFRCOCAL1_SPEC;
impl crate::RegisterSpec for AUXHFRCOCAL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`auxhfrcocal1::R`](R) reader structure"]
impl crate::Readable for AUXHFRCOCAL1_SPEC {}
