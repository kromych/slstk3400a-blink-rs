#[doc = "Register `CACHEHITS` reader"]
pub type R = crate::R<CachehitsSpec>;
#[doc = "Field `CACHEHITS` reader - Cache hits since last performance counter start command."]
pub type CachehitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Cache hits since last performance counter start command."]
    #[inline(always)]
    pub fn cachehits(&self) -> CachehitsR {
        CachehitsR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "Cache Hits Performance Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cachehits::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CachehitsSpec;
impl crate::RegisterSpec for CachehitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cachehits::R`](R) reader structure"]
impl crate::Readable for CachehitsSpec {}
#[doc = "`reset()` method sets CACHEHITS to value 0"]
impl crate::Resettable for CachehitsSpec {}
