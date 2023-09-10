#[doc = "Register `CACHEHITS` reader"]
pub type R = crate::R<CACHEHITS_SPEC>;
#[doc = "Field `CACHEHITS` reader - Cache hits since last performance counter start command."]
pub type CACHEHITS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Cache hits since last performance counter start command."]
    #[inline(always)]
    pub fn cachehits(&self) -> CACHEHITS_R {
        CACHEHITS_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "Cache Hits Performance Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cachehits::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHEHITS_SPEC;
impl crate::RegisterSpec for CACHEHITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cachehits::R`](R) reader structure"]
impl crate::Readable for CACHEHITS_SPEC {}
#[doc = "`reset()` method sets CACHEHITS to value 0"]
impl crate::Resettable for CACHEHITS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
