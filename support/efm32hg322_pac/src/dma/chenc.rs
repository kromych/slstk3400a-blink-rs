#[doc = "Register `CHENC` writer"]
pub type W = crate::W<CHENC_SPEC>;
#[doc = "Field `CH0ENC` writer - Channel 0 Enable Clear"]
pub type CH0ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ENC` writer - Channel 1 Enable Clear"]
pub type CH1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ENC` writer - Channel 2 Enable Clear"]
pub type CH2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ENC` writer - Channel 3 Enable Clear"]
pub type CH3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4ENC` writer - Channel 4 Enable Clear"]
pub type CH4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5ENC` writer - Channel 5 Enable Clear"]
pub type CH5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0enc(&mut self) -> CH0ENC_W<CHENC_SPEC> {
        CH0ENC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1enc(&mut self) -> CH1ENC_W<CHENC_SPEC> {
        CH1ENC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2enc(&mut self) -> CH2ENC_W<CHENC_SPEC> {
        CH2ENC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3enc(&mut self) -> CH3ENC_W<CHENC_SPEC> {
        CH3ENC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4enc(&mut self) -> CH4ENC_W<CHENC_SPEC> {
        CH4ENC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5enc(&mut self) -> CH5ENC_W<CHENC_SPEC> {
        CH5ENC_W::new(self, 5)
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
#[doc = "Channel Enable Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chenc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHENC_SPEC;
impl crate::RegisterSpec for CHENC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chenc::W`](W) writer structure"]
impl crate::Writable for CHENC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHENC to value 0"]
impl crate::Resettable for CHENC_SPEC {
    const RESET_VALUE: u32 = 0;
}
