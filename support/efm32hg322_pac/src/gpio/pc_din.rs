#[doc = "Register `PC_DIN` reader"]
pub type R = crate::R<PcDinSpec>;
#[doc = "Field `DIN` reader - Data In"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data In"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcDinSpec;
impl crate::RegisterSpec for PcDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_din::R`](R) reader structure"]
impl crate::Readable for PcDinSpec {}
#[doc = "`reset()` method sets PC_DIN to value 0"]
impl crate::Resettable for PcDinSpec {}
