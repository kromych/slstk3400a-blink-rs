#[doc = "Register `PF_DIN` reader"]
pub type R = crate::R<PfDinSpec>;
#[doc = "Field `DIN` reader - Data In"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data In"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfDinSpec;
impl crate::RegisterSpec for PfDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_din::R`](R) reader structure"]
impl crate::Readable for PfDinSpec {}
#[doc = "`reset()` method sets PF_DIN to value 0"]
impl crate::Resettable for PfDinSpec {}
