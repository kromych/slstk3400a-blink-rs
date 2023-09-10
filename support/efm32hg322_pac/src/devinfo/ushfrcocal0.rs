#[doc = "Register `USHFRCOCAL0` reader"]
pub type R = crate::R<USHFRCOCAL0_SPEC>;
#[doc = "Field `BAND24_TUNING` reader - 24 MHz TUNING value for USFRCO"]
pub type BAND24_TUNING_R = crate::FieldReader;
#[doc = "Field `BAND24_FINETUNING` reader - 24 MHz FINETUNING value for USFRCO"]
pub type BAND24_FINETUNING_R = crate::FieldReader;
#[doc = "Field `BAND48_TUNING` reader - 24 MHz TUNING value for USFRCO"]
pub type BAND48_TUNING_R = crate::FieldReader;
#[doc = "Field `BAND48_FINETUNING` reader - 24 MHz FINETUNING value for USFRCO"]
pub type BAND48_FINETUNING_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_tuning(&self) -> BAND24_TUNING_R {
        BAND24_TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_finetuning(&self) -> BAND24_FINETUNING_R {
        BAND24_FINETUNING_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_tuning(&self) -> BAND48_TUNING_R {
        BAND48_TUNING_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_finetuning(&self) -> BAND48_FINETUNING_R {
        BAND48_FINETUNING_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "USHFRCO calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcocal0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USHFRCOCAL0_SPEC;
impl crate::RegisterSpec for USHFRCOCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcocal0::R`](R) reader structure"]
impl crate::Readable for USHFRCOCAL0_SPEC {}
