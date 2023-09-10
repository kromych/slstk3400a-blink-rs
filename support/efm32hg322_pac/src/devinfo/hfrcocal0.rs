#[doc = "Register `HFRCOCAL0` reader"]
pub type R = crate::R<HFRCOCAL0_SPEC>;
#[doc = "Field `BAND1` reader - 1MHz tuning value for HFRCO"]
pub type BAND1_R = crate::FieldReader;
#[doc = "Field `BAND7` reader - 7MHz tuning value for HFRCO"]
pub type BAND7_R = crate::FieldReader;
#[doc = "Field `BAND11` reader - 11MHz tuning value for HFRCO"]
pub type BAND11_R = crate::FieldReader;
#[doc = "Field `BAND14` reader - 14MHz tuning value for HFRCO"]
pub type BAND14_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 1MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band1(&self) -> BAND1_R {
        BAND1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 7MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band7(&self) -> BAND7_R {
        BAND7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 11MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band11(&self) -> BAND11_R {
        BAND11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 14MHz tuning value for HFRCO"]
    #[inline(always)]
    pub fn band14(&self) -> BAND14_R {
        BAND14_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "HFRCO calibration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfrcocal0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFRCOCAL0_SPEC;
impl crate::RegisterSpec for HFRCOCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrcocal0::R`](R) reader structure"]
impl crate::Readable for HFRCOCAL0_SPEC {}
