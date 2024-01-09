#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `SINGLESTART` writer - Single Conversion Start"]
pub type SINGLESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLESTOP` writer - Single Conversion Stop"]
pub type SINGLESTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANSTART` writer - Scan Sequence Start"]
pub type SCANSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANSTOP` writer - Scan Sequence Stop"]
pub type SCANSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Single Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn singlestart(&mut self) -> SINGLESTART_W<CMD_SPEC> {
        SINGLESTART_W::new(self, 0)
    }
    #[doc = "Bit 1 - Single Conversion Stop"]
    #[inline(always)]
    #[must_use]
    pub fn singlestop(&mut self) -> SINGLESTOP_W<CMD_SPEC> {
        SINGLESTOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Scan Sequence Start"]
    #[inline(always)]
    #[must_use]
    pub fn scanstart(&mut self) -> SCANSTART_W<CMD_SPEC> {
        SCANSTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - Scan Sequence Stop"]
    #[inline(always)]
    #[must_use]
    pub fn scanstop(&mut self) -> SCANSTOP_W<CMD_SPEC> {
        SCANSTOP_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
