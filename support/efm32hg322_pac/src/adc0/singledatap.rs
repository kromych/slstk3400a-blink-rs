#[doc = "Register `SINGLEDATAP` reader"]
pub type R = crate::R<SingledatapSpec>;
#[doc = "Field `DATAP` reader - Single Conversion Result Data Peek"]
pub type DatapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Single Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DatapR {
        DatapR::new(self.bits)
    }
}
#[doc = "Single Conversion Result Data Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singledatap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingledatapSpec;
impl crate::RegisterSpec for SingledatapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singledatap::R`](R) reader structure"]
impl crate::Readable for SingledatapSpec {}
#[doc = "`reset()` method sets SINGLEDATAP to value 0"]
impl crate::Resettable for SingledatapSpec {}
