#[doc = "Register `TOPB` reader"]
pub type R = crate::R<TOPB_SPEC>;
#[doc = "Register `TOPB` writer"]
pub type W = crate::W<TOPB_SPEC>;
#[doc = "Field `TOPB` reader - Counter Top Buffer"]
pub type TOPB_R = crate::FieldReader<u16>;
#[doc = "Field `TOPB` writer - Counter Top Buffer"]
pub type TOPB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Buffer"]
    #[inline(always)]
    pub fn topb(&self) -> TOPB_R {
        TOPB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Top Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn topb(&mut self) -> TOPB_W<TOPB_SPEC, 0> {
        TOPB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Top Value Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`topb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`topb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOPB_SPEC;
impl crate::RegisterSpec for TOPB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`topb::R`](R) reader structure"]
impl crate::Readable for TOPB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`topb::W`](W) writer structure"]
impl crate::Writable for TOPB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOPB to value 0xff"]
impl crate::Resettable for TOPB_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
