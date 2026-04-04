#[doc = "Register `HFRCOCAL0` reader"]
pub type R = crate::R<Hfrcocal0Spec>;
#[doc = "Field `BAND1` reader - 1MHz tuning value for HFRCO"]
pub type Band1R = crate::FieldReader;
#[doc = "Field `BAND7` reader - 7MHz tuning value for HFRCO"]
pub type Band7R = crate::FieldReader;
#[doc = "Field `BAND11` reader - 11MHz tuning value for HFRCO"]
pub type Band11R = crate::FieldReader;
#[doc = "Field `BAND14` reader - 14MHz tuning value for HFRCO"]
pub type Band14R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 1MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band1(&self) -> Band1R {
        Band1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 7MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band7(&self) -> Band7R {
        Band7R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 11MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band11(&self) -> Band11R {
        Band11R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 14MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band14(&self) -> Band14R {
        Band14R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "HFRCO calibration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcocal0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfrcocal0Spec;
impl crate::RegisterSpec for Hfrcocal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrcocal0::R`](R) reader structure"]
impl crate::Readable for Hfrcocal0Spec {}
