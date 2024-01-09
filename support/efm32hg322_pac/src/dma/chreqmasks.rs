#[doc = "Register `CHREQMASKS` writer"]
pub type W = crate::W<CHREQMASKS_SPEC>;
#[doc = "Field `CH0REQMASKS` writer - Channel 0 Request Mask Set"]
pub type CH0REQMASKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1REQMASKS` writer - Channel 1 Request Mask Set"]
pub type CH1REQMASKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2REQMASKS` writer - Channel 2 Request Mask Set"]
pub type CH2REQMASKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3REQMASKS` writer - Channel 3 Request Mask Set"]
pub type CH3REQMASKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4REQMASKS` writer - Channel 4 Request Mask Set"]
pub type CH4REQMASKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5REQMASKS` writer - Channel 5 Request Mask Set"]
pub type CH5REQMASKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0reqmasks(&mut self) -> CH0REQMASKS_W<CHREQMASKS_SPEC> {
        CH0REQMASKS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1reqmasks(&mut self) -> CH1REQMASKS_W<CHREQMASKS_SPEC> {
        CH1REQMASKS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2reqmasks(&mut self) -> CH2REQMASKS_W<CHREQMASKS_SPEC> {
        CH2REQMASKS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3reqmasks(&mut self) -> CH3REQMASKS_W<CHREQMASKS_SPEC> {
        CH3REQMASKS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4reqmasks(&mut self) -> CH4REQMASKS_W<CHREQMASKS_SPEC> {
        CH4REQMASKS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5reqmasks(&mut self) -> CH5REQMASKS_W<CHREQMASKS_SPEC> {
        CH5REQMASKS_W::new(self, 5)
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
#[doc = "Channel Request Mask Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chreqmasks::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHREQMASKS_SPEC;
impl crate::RegisterSpec for CHREQMASKS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chreqmasks::W`](W) writer structure"]
impl crate::Writable for CHREQMASKS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHREQMASKS to value 0"]
impl crate::Resettable for CHREQMASKS_SPEC {
    const RESET_VALUE: u32 = 0;
}
