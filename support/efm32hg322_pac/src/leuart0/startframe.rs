#[doc = "Register `STARTFRAME` reader"]
pub type R = crate::R<STARTFRAME_SPEC>;
#[doc = "Register `STARTFRAME` writer"]
pub type W = crate::W<STARTFRAME_SPEC>;
#[doc = "Field `STARTFRAME` reader - Start Frame"]
pub type STARTFRAME_R = crate::FieldReader<u16>;
#[doc = "Field `STARTFRAME` writer - Start Frame"]
pub type STARTFRAME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    pub fn startframe(&self) -> STARTFRAME_R {
        STARTFRAME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    #[must_use]
    pub fn startframe(&mut self) -> STARTFRAME_W<STARTFRAME_SPEC, 0> {
        STARTFRAME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Start Frame Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startframe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startframe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STARTFRAME_SPEC;
impl crate::RegisterSpec for STARTFRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startframe::R`](R) reader structure"]
impl crate::Readable for STARTFRAME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`startframe::W`](W) writer structure"]
impl crate::Writable for STARTFRAME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STARTFRAME to value 0"]
impl crate::Resettable for STARTFRAME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
