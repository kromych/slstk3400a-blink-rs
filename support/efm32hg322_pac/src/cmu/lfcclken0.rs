#[doc = "Register `LFCCLKEN0` reader"]
pub type R = crate::R<LFCCLKEN0_SPEC>;
#[doc = "Register `LFCCLKEN0` writer"]
pub type W = crate::W<LFCCLKEN0_SPEC>;
#[doc = "Field `USBLE` reader - Universal Serial Bus Low Energy Clock Clock Enable"]
pub type USBLE_R = crate::BitReader;
#[doc = "Field `USBLE` writer - Universal Serial Bus Low Energy Clock Clock Enable"]
pub type USBLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Universal Serial Bus Low Energy Clock Clock Enable"]
    #[inline(always)]
    pub fn usble(&self) -> USBLE_R {
        USBLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Serial Bus Low Energy Clock Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usble(&mut self) -> USBLE_W<LFCCLKEN0_SPEC, 0> {
        USBLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfcclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfcclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFCCLKEN0_SPEC;
impl crate::RegisterSpec for LFCCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfcclken0::R`](R) reader structure"]
impl crate::Readable for LFCCLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfcclken0::W`](W) writer structure"]
impl crate::Writable for LFCCLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFCCLKEN0 to value 0"]
impl crate::Resettable for LFCCLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
