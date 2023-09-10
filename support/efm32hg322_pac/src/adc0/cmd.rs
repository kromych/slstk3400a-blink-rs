#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `SINGLESTART` writer - Single Conversion Start"]
pub type SINGLESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SINGLESTOP` writer - Single Conversion Stop"]
pub type SINGLESTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANSTART` writer - Scan Sequence Start"]
pub type SCANSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANSTOP` writer - Scan Sequence Stop"]
pub type SCANSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Single Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn singlestart(&mut self) -> SINGLESTART_W<CMD_SPEC, 0> {
        SINGLESTART_W::new(self)
    }
    #[doc = "Bit 1 - Single Conversion Stop"]
    #[inline(always)]
    #[must_use]
    pub fn singlestop(&mut self) -> SINGLESTOP_W<CMD_SPEC, 1> {
        SINGLESTOP_W::new(self)
    }
    #[doc = "Bit 2 - Scan Sequence Start"]
    #[inline(always)]
    #[must_use]
    pub fn scanstart(&mut self) -> SCANSTART_W<CMD_SPEC, 2> {
        SCANSTART_W::new(self)
    }
    #[doc = "Bit 3 - Scan Sequence Stop"]
    #[inline(always)]
    #[must_use]
    pub fn scanstop(&mut self) -> SCANSTOP_W<CMD_SPEC, 3> {
        SCANSTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
