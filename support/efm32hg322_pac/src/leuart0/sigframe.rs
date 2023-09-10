#[doc = "Register `SIGFRAME` reader"]
pub type R = crate::R<SIGFRAME_SPEC>;
#[doc = "Register `SIGFRAME` writer"]
pub type W = crate::W<SIGFRAME_SPEC>;
#[doc = "Field `SIGFRAME` reader - Signal Frame"]
pub type SIGFRAME_R = crate::FieldReader<u16>;
#[doc = "Field `SIGFRAME` writer - Signal Frame"]
pub type SIGFRAME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    pub fn sigframe(&self) -> SIGFRAME_R {
        SIGFRAME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sigframe(&mut self) -> SIGFRAME_W<SIGFRAME_SPEC, 0> {
        SIGFRAME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Signal Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigframe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigframe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGFRAME_SPEC;
impl crate::RegisterSpec for SIGFRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigframe::R`](R) reader structure"]
impl crate::Readable for SIGFRAME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigframe::W`](W) writer structure"]
impl crate::Writable for SIGFRAME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGFRAME to value 0"]
impl crate::Resettable for SIGFRAME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
