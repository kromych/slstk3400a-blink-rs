#[doc = "Register `CTRLBASE` reader"]
pub type R = crate::R<CTRLBASE_SPEC>;
#[doc = "Register `CTRLBASE` writer"]
pub type W = crate::W<CTRLBASE_SPEC>;
#[doc = "Field `CTRLBASE` reader - Channel Control Data Base Pointer"]
pub type CTRLBASE_R = crate::FieldReader<u32>;
#[doc = "Field `CTRLBASE` writer - Channel Control Data Base Pointer"]
pub type CTRLBASE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    pub fn ctrlbase(&self) -> CTRLBASE_R {
        CTRLBASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlbase(&mut self) -> CTRLBASE_W<CTRLBASE_SPEC, 0> {
        CTRLBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Control Data Base Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlbase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlbase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLBASE_SPEC;
impl crate::RegisterSpec for CTRLBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlbase::R`](R) reader structure"]
impl crate::Readable for CTRLBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlbase::W`](W) writer structure"]
impl crate::Writable for CTRLBASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLBASE to value 0"]
impl crate::Resettable for CTRLBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
