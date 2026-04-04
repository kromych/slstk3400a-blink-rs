#[doc = "Register `SCANDATAP` reader"]
pub type R = crate::R<ScandatapSpec>;
#[doc = "Field `DATAP` reader - Scan Conversion Result Data Peek"]
pub type DatapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DatapR {
        DatapR::new(self.bits)
    }
}
#[doc = "Scan Sequence Result Data Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scandatap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScandatapSpec;
impl crate::RegisterSpec for ScandatapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandatap::R`](R) reader structure"]
impl crate::Readable for ScandatapSpec {}
#[doc = "`reset()` method sets SCANDATAP to value 0"]
impl crate::Resettable for ScandatapSpec {}
