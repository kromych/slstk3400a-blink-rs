#[doc = "Register `CHREQMASKC` writer"]
pub type W = crate::W<CHREQMASKC_SPEC>;
#[doc = "Field `CH0REQMASKC` writer - Channel 0 Request Mask Clear"]
pub type CH0REQMASKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1REQMASKC` writer - Channel 1 Request Mask Clear"]
pub type CH1REQMASKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2REQMASKC` writer - Channel 2 Request Mask Clear"]
pub type CH2REQMASKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3REQMASKC` writer - Channel 3 Request Mask Clear"]
pub type CH3REQMASKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4REQMASKC` writer - Channel 4 Request Mask Clear"]
pub type CH4REQMASKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5REQMASKC` writer - Channel 5 Request Mask Clear"]
pub type CH5REQMASKC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0reqmaskc(&mut self) -> CH0REQMASKC_W<CHREQMASKC_SPEC, 0> {
        CH0REQMASKC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1reqmaskc(&mut self) -> CH1REQMASKC_W<CHREQMASKC_SPEC, 1> {
        CH1REQMASKC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2reqmaskc(&mut self) -> CH2REQMASKC_W<CHREQMASKC_SPEC, 2> {
        CH2REQMASKC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3reqmaskc(&mut self) -> CH3REQMASKC_W<CHREQMASKC_SPEC, 3> {
        CH3REQMASKC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4reqmaskc(&mut self) -> CH4REQMASKC_W<CHREQMASKC_SPEC, 4> {
        CH4REQMASKC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5reqmaskc(&mut self) -> CH5REQMASKC_W<CHREQMASKC_SPEC, 5> {
        CH5REQMASKC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Request Mask Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chreqmaskc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHREQMASKC_SPEC;
impl crate::RegisterSpec for CHREQMASKC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chreqmaskc::W`](W) writer structure"]
impl crate::Writable for CHREQMASKC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHREQMASKC to value 0"]
impl crate::Resettable for CHREQMASKC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
