#[doc = "Register `UNIQUEL` reader"]
pub type R = crate::R<UNIQUEL_SPEC>;
#[doc = "Field `UNIQUEL` reader - Lower part of 64-bit device unique number"]
pub type UNIQUEL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Lower part of 64-bit device unique number"]
    #[inline(always)]
    pub fn uniquel(&self) -> UNIQUEL_R {
        UNIQUEL_R::new(self.bits)
    }
}
#[doc = "Low 32 bits of device unique number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uniquel::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIQUEL_SPEC;
impl crate::RegisterSpec for UNIQUEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uniquel::R`](R) reader structure"]
impl crate::Readable for UNIQUEL_SPEC {}
