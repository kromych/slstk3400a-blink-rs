#[doc = "Register `CACHEMISSES` reader"]
pub type R = crate::R<CACHEMISSES_SPEC>;
#[doc = "Field `CACHEMISSES` reader - Cache misses since last performance counter start command."]
pub type CACHEMISSES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Cache misses since last performance counter start command."]
    #[inline(always)]
    pub fn cachemisses(&self) -> CACHEMISSES_R {
        CACHEMISSES_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "Cache Misses Performance Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cachemisses::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHEMISSES_SPEC;
impl crate::RegisterSpec for CACHEMISSES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cachemisses::R`](R) reader structure"]
impl crate::Readable for CACHEMISSES_SPEC {}
#[doc = "`reset()` method sets CACHEMISSES to value 0"]
impl crate::Resettable for CACHEMISSES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
