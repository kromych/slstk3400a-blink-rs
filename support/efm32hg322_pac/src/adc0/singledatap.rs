#[doc = "Register `SINGLEDATAP` reader"]
pub type R = crate::R<SINGLEDATAP_SPEC>;
#[doc = "Field `DATAP` reader - Single Conversion Result Data Peek"]
pub type DATAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Single Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new(self.bits)
    }
}
#[doc = "Single Conversion Result Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singledatap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLEDATAP_SPEC;
impl crate::RegisterSpec for SINGLEDATAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singledatap::R`](R) reader structure"]
impl crate::Readable for SINGLEDATAP_SPEC {}
#[doc = "`reset()` method sets SINGLEDATAP to value 0"]
impl crate::Resettable for SINGLEDATAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
