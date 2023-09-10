#[doc = "Register `CHENC` writer"]
pub type W = crate::W<CHENC_SPEC>;
#[doc = "Field `CH0ENC` writer - Channel 0 Enable Clear"]
pub type CH0ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1ENC` writer - Channel 1 Enable Clear"]
pub type CH1ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2ENC` writer - Channel 2 Enable Clear"]
pub type CH2ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3ENC` writer - Channel 3 Enable Clear"]
pub type CH3ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4ENC` writer - Channel 4 Enable Clear"]
pub type CH4ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5ENC` writer - Channel 5 Enable Clear"]
pub type CH5ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0enc(&mut self) -> CH0ENC_W<CHENC_SPEC, 0> {
        CH0ENC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1enc(&mut self) -> CH1ENC_W<CHENC_SPEC, 1> {
        CH1ENC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2enc(&mut self) -> CH2ENC_W<CHENC_SPEC, 2> {
        CH2ENC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3enc(&mut self) -> CH3ENC_W<CHENC_SPEC, 3> {
        CH3ENC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4enc(&mut self) -> CH4ENC_W<CHENC_SPEC, 4> {
        CH4ENC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5enc(&mut self) -> CH5ENC_W<CHENC_SPEC, 5> {
        CH5ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Enable Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chenc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHENC_SPEC;
impl crate::RegisterSpec for CHENC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chenc::W`](W) writer structure"]
impl crate::Writable for CHENC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHENC to value 0"]
impl crate::Resettable for CHENC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
