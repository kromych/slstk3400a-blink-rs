#[doc = "Register `AUXCNT` reader"]
pub type R = crate::R<AUXCNT_SPEC>;
#[doc = "Register `AUXCNT` writer"]
pub type W = crate::W<AUXCNT_SPEC>;
#[doc = "Field `AUXCNT` reader - Auxiliary Counter Value"]
pub type AUXCNT_R = crate::FieldReader<u16>;
#[doc = "Field `AUXCNT` writer - Auxiliary Counter Value"]
pub type AUXCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AUXCNT_R {
        AUXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn auxcnt(&mut self) -> AUXCNT_W<AUXCNT_SPEC, 0> {
        AUXCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Auxiliary Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUXCNT_SPEC;
impl crate::RegisterSpec for AUXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxcnt::R`](R) reader structure"]
impl crate::Readable for AUXCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`auxcnt::W`](W) writer structure"]
impl crate::Writable for AUXCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXCNT to value 0"]
impl crate::Resettable for AUXCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
