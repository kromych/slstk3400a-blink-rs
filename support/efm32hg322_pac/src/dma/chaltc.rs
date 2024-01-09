#[doc = "Register `CHALTC` writer"]
pub type W = crate::W<CHALTC_SPEC>;
#[doc = "Field `CH0ALTC` writer - Channel 0 Alternate Clear"]
pub type CH0ALTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ALTC` writer - Channel 1 Alternate Clear"]
pub type CH1ALTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ALTC` writer - Channel 2 Alternate Clear"]
pub type CH2ALTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ALTC` writer - Channel 3 Alternate Clear"]
pub type CH3ALTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4ALTC` writer - Channel 4 Alternate Clear"]
pub type CH4ALTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5ALTC` writer - Channel 5 Alternate Clear"]
pub type CH5ALTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0altc(&mut self) -> CH0ALTC_W<CHALTC_SPEC> {
        CH0ALTC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1altc(&mut self) -> CH1ALTC_W<CHALTC_SPEC> {
        CH1ALTC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2altc(&mut self) -> CH2ALTC_W<CHALTC_SPEC> {
        CH2ALTC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3altc(&mut self) -> CH3ALTC_W<CHALTC_SPEC> {
        CH3ALTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Alternate Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4altc(&mut self) -> CH4ALTC_W<CHALTC_SPEC> {
        CH4ALTC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Alternate Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5altc(&mut self) -> CH5ALTC_W<CHALTC_SPEC> {
        CH5ALTC_W::new(self, 5)
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
#[doc = "Channel Alternate Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chaltc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHALTC_SPEC;
impl crate::RegisterSpec for CHALTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chaltc::W`](W) writer structure"]
impl crate::Writable for CHALTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHALTC to value 0"]
impl crate::Resettable for CHALTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
