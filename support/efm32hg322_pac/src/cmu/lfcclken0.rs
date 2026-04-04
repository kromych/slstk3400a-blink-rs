#[doc = "Register `LFCCLKEN0` reader"]
pub type R = crate::R<Lfcclken0Spec>;
#[doc = "Register `LFCCLKEN0` writer"]
pub type W = crate::W<Lfcclken0Spec>;
#[doc = "Field `USBLE` reader - Universal Serial Bus Low Energy Clock Clock Enable"]
pub type UsbleR = crate::BitReader;
#[doc = "Field `USBLE` writer - Universal Serial Bus Low Energy Clock Clock Enable"]
pub type UsbleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Universal Serial Bus Low Energy Clock Clock Enable"]
    #[inline(always)]
    pub fn usble(&self) -> UsbleR {
        UsbleR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Serial Bus Low Energy Clock Clock Enable"]
    #[inline(always)]
    pub fn usble(&mut self) -> UsbleW<'_, Lfcclken0Spec> {
        UsbleW::new(self, 0)
    }
}
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfcclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfcclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfcclken0Spec;
impl crate::RegisterSpec for Lfcclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfcclken0::R`](R) reader structure"]
impl crate::Readable for Lfcclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfcclken0::W`](W) writer structure"]
impl crate::Writable for Lfcclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFCCLKEN0 to value 0"]
impl crate::Resettable for Lfcclken0Spec {}
