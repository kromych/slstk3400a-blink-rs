#[doc = "Register `SCANDATA` reader"]
pub type R = crate::R<SCANDATA_SPEC>;
#[doc = "Field `DATA` reader - Scan Conversion Result Data"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Scan Conversion Result Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandata::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANDATA_SPEC;
impl crate::RegisterSpec for SCANDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandata::R`](R) reader structure"]
impl crate::Readable for SCANDATA_SPEC {}
#[doc = "`reset()` method sets SCANDATA to value 0"]
impl crate::Resettable for SCANDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
