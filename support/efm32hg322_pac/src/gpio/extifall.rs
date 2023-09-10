#[doc = "Register `EXTIFALL` reader"]
pub type R = crate::R<EXTIFALL_SPEC>;
#[doc = "Register `EXTIFALL` writer"]
pub type W = crate::W<EXTIFALL_SPEC>;
#[doc = "Field `EXTIFALL` reader - External Interrupt n Falling Edge Trigger Enable"]
pub type EXTIFALL_R = crate::FieldReader<u16>;
#[doc = "Field `EXTIFALL` writer - External Interrupt n Falling Edge Trigger Enable"]
pub type EXTIFALL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt n Falling Edge Trigger Enable"]
    #[inline(always)]
    pub fn extifall(&self) -> EXTIFALL_R {
        EXTIFALL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt n Falling Edge Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extifall(&mut self) -> EXTIFALL_W<EXTIFALL_SPEC, 0> {
        EXTIFALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extifall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extifall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIFALL_SPEC;
impl crate::RegisterSpec for EXTIFALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extifall::R`](R) reader structure"]
impl crate::Readable for EXTIFALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extifall::W`](W) writer structure"]
impl crate::Writable for EXTIFALL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIFALL to value 0"]
impl crate::Resettable for EXTIFALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
