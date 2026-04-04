#[doc = "Register `USHFRCOCAL0` reader"]
pub type R = crate::R<Ushfrcocal0Spec>;
#[doc = "Field `BAND24_TUNING` reader - 24 MHz TUNING value for USFRCO"]
pub type Band24TuningR = crate::FieldReader;
#[doc = "Field `BAND24_FINETUNING` reader - 24 MHz FINETUNING value for USFRCO"]
pub type Band24FinetuningR = crate::FieldReader;
#[doc = "Field `BAND48_TUNING` reader - 24 MHz TUNING value for USFRCO"]
pub type Band48TuningR = crate::FieldReader;
#[doc = "Field `BAND48_FINETUNING` reader - 24 MHz FINETUNING value for USFRCO"]
pub type Band48FinetuningR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_tuning(&self) -> Band24TuningR {
        Band24TuningR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_finetuning(&self) -> Band24FinetuningR {
        Band24FinetuningR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_tuning(&self) -> Band48TuningR {
        Band48TuningR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_finetuning(&self) -> Band48FinetuningR {
        Band48FinetuningR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "USHFRCO calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcocal0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ushfrcocal0Spec;
impl crate::RegisterSpec for Ushfrcocal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcocal0::R`](R) reader structure"]
impl crate::Readable for Ushfrcocal0Spec {}
