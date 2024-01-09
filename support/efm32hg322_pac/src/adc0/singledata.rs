#[doc = "Register `SINGLEDATA` reader"]
pub type R = crate::R<SINGLEDATA_SPEC>;
#[doc = "Field `DATA` reader - Single Conversion Result Data"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Single Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Single Conversion Result Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singledata::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLEDATA_SPEC;
impl crate::RegisterSpec for SINGLEDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singledata::R`](R) reader structure"]
impl crate::Readable for SINGLEDATA_SPEC {}
#[doc = "`reset()` method sets SINGLEDATA to value 0"]
impl crate::Resettable for SINGLEDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
