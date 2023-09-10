#[doc = "Register `PA_DIN` reader"]
pub type R = crate::R<PA_DIN_SPEC>;
#[doc = "Field `DIN` reader - Data In"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data In"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA_DIN_SPEC;
impl crate::RegisterSpec for PA_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_din::R`](R) reader structure"]
impl crate::Readable for PA_DIN_SPEC {}
#[doc = "`reset()` method sets PA_DIN to value 0"]
impl crate::Resettable for PA_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
