#[doc = "Register `CACHEMISSES` reader"]
pub type R = crate::R<CachemissesSpec>;
#[doc = "Field `CACHEMISSES` reader - Cache misses since last performance counter start command."]
pub type CachemissesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Cache misses since last performance counter start command."]
    #[inline(always)]
    pub fn cachemisses(&self) -> CachemissesR {
        CachemissesR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "Cache Misses Performance Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cachemisses::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CachemissesSpec;
impl crate::RegisterSpec for CachemissesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cachemisses::R`](R) reader structure"]
impl crate::Readable for CachemissesSpec {}
#[doc = "`reset()` method sets CACHEMISSES to value 0"]
impl crate::Resettable for CachemissesSpec {}
