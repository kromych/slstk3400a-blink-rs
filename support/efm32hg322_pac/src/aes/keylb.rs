#[doc = "Register `KEYLB` reader"]
pub type R = crate::R<KEYLB_SPEC>;
#[doc = "Register `KEYLB` writer"]
pub type W = crate::W<KEYLB_SPEC>;
#[doc = "Field `KEYLB` reader - Key Low Access B"]
pub type KEYLB_R = crate::FieldReader<u32>;
#[doc = "Field `KEYLB` writer - Key Low Access B"]
pub type KEYLB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access B"]
    #[inline(always)]
    pub fn keylb(&self) -> KEYLB_R {
        KEYLB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access B"]
    #[inline(always)]
    #[must_use]
    pub fn keylb(&mut self) -> KEYLB_W<KEYLB_SPEC, 0> {
        KEYLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "KEY Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keylb::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keylb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYLB_SPEC;
impl crate::RegisterSpec for KEYLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keylb::R`](R) reader structure"]
impl crate::Readable for KEYLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`keylb::W`](W) writer structure"]
impl crate::Writable for KEYLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLB to value 0"]
impl crate::Resettable for KEYLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
