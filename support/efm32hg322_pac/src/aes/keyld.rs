#[doc = "Register `KEYLD` reader"]
pub type R = crate::R<KEYLD_SPEC>;
#[doc = "Register `KEYLD` writer"]
pub type W = crate::W<KEYLD_SPEC>;
#[doc = "Field `KEYLD` reader - Key Low Access D"]
pub type KEYLD_R = crate::FieldReader<u32>;
#[doc = "Field `KEYLD` writer - Key Low Access D"]
pub type KEYLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Key Low Access D"]
    #[inline(always)]
    pub fn keyld(&self) -> KEYLD_R {
        KEYLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access D"]
    #[inline(always)]
    #[must_use]
    pub fn keyld(&mut self) -> KEYLD_W<KEYLD_SPEC, 0> {
        KEYLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "KEY Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyld::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYLD_SPEC;
impl crate::RegisterSpec for KEYLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyld::R`](R) reader structure"]
impl crate::Readable for KEYLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`keyld::W`](W) writer structure"]
impl crate::Writable for KEYLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLD to value 0"]
impl crate::Resettable for KEYLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
