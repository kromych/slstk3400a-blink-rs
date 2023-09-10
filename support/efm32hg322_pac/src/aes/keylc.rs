#[doc = "Register `KEYLC` reader"]
pub type R = crate::R<KEYLC_SPEC>;
#[doc = "Register `KEYLC` writer"]
pub type W = crate::W<KEYLC_SPEC>;
#[doc = "Field `KEYLC` reader - Key Low Access C"]
pub type KEYLC_R = crate::FieldReader<u32>;
#[doc = "Field `KEYLC` writer - Key Low Access C"]
pub type KEYLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    pub fn keylc(&self) -> KEYLC_R {
        KEYLC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access C"]
    #[inline(always)]
    #[must_use]
    pub fn keylc(&mut self) -> KEYLC_W<KEYLC_SPEC, 0> {
        KEYLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "KEY Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keylc::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keylc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYLC_SPEC;
impl crate::RegisterSpec for KEYLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keylc::R`](R) reader structure"]
impl crate::Readable for KEYLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`keylc::W`](W) writer structure"]
impl crate::Writable for KEYLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLC to value 0"]
impl crate::Resettable for KEYLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
