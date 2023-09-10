#[doc = "Register `PF_DIN` reader"]
pub type R = crate::R<PF_DIN_SPEC>;
#[doc = "Field `DIN` reader - Data In"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data In"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_DIN_SPEC;
impl crate::RegisterSpec for PF_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_din::R`](R) reader structure"]
impl crate::Readable for PF_DIN_SPEC {}
#[doc = "`reset()` method sets PF_DIN to value 0"]
impl crate::Resettable for PF_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
