#[doc = "Register `DIEP0DMAADDR` reader"]
pub type R = crate::R<DIEP0DMAADDR_SPEC>;
#[doc = "Register `DIEP0DMAADDR` writer"]
pub type W = crate::W<DIEP0DMAADDR_SPEC>;
#[doc = "Field `DIEP0DMAADDR` reader - DMA Address"]
pub type DIEP0DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DIEP0DMAADDR` writer - DMA Address"]
pub type DIEP0DMAADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn diep0dmaaddr(&self) -> DIEP0DMAADDR_R {
        DIEP0DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn diep0dmaaddr(&mut self) -> DIEP0DMAADDR_W<DIEP0DMAADDR_SPEC, 0> {
        DIEP0DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device IN Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0dmaaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0dmaaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP0DMAADDR_SPEC;
impl crate::RegisterSpec for DIEP0DMAADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0dmaaddr::R`](R) reader structure"]
impl crate::Readable for DIEP0DMAADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep0dmaaddr::W`](W) writer structure"]
impl crate::Writable for DIEP0DMAADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0DMAADDR to value 0"]
impl crate::Resettable for DIEP0DMAADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
