#[doc = "Register `UNIQUEH` reader"]
pub type R = crate::R<UNIQUEH_SPEC>;
#[doc = "Field `UNIQUEH` reader - High part of 64-bit device unique number"]
pub type UNIQUEH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - High part of 64-bit device unique number"]
    #[inline(always)]
    pub fn uniqueh(&self) -> UNIQUEH_R {
        UNIQUEH_R::new(self.bits)
    }
}
#[doc = "High 32 bits of device unique number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uniqueh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIQUEH_SPEC;
impl crate::RegisterSpec for UNIQUEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uniqueh::R`](R) reader structure"]
impl crate::Readable for UNIQUEH_SPEC {}
