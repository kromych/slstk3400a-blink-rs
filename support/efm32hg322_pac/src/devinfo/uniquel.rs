#[doc = "Register `UNIQUEL` reader"]
pub type R = crate::R<UniquelSpec>;
#[doc = "Field `UNIQUEL` reader - Lower part of 64-bit device unique number"]
pub type UniquelR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Lower part of 64-bit device unique number"]
    #[inline(always)]
    pub fn uniquel(&self) -> UniquelR {
        UniquelR::new(self.bits)
    }
}
#[doc = "Low 32 bits of device unique number\n\nYou can [`read`](crate::Reg::read) this register and get [`uniquel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UniquelSpec;
impl crate::RegisterSpec for UniquelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uniquel::R`](R) reader structure"]
impl crate::Readable for UniquelSpec {}
