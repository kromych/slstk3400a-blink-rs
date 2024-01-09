#[doc = "Register `CHENS` writer"]
pub type W = crate::W<CHENS_SPEC>;
#[doc = "Field `CH0ENS` writer - Channel 0 Enable Set"]
pub type CH0ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ENS` writer - Channel 1 Enable Set"]
pub type CH1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ENS` writer - Channel 2 Enable Set"]
pub type CH2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ENS` writer - Channel 3 Enable Set"]
pub type CH3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4ENS` writer - Channel 4 Enable Set"]
pub type CH4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5ENS` writer - Channel 5 Enable Set"]
pub type CH5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ens(&mut self) -> CH0ENS_W<CHENS_SPEC> {
        CH0ENS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ens(&mut self) -> CH1ENS_W<CHENS_SPEC> {
        CH1ENS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ens(&mut self) -> CH2ENS_W<CHENS_SPEC> {
        CH2ENS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ens(&mut self) -> CH3ENS_W<CHENS_SPEC> {
        CH3ENS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4ens(&mut self) -> CH4ENS_W<CHENS_SPEC> {
        CH4ENS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5ens(&mut self) -> CH5ENS_W<CHENS_SPEC> {
        CH5ENS_W::new(self, 5)
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
#[doc = "Channel Enable Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chens::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHENS_SPEC;
impl crate::RegisterSpec for CHENS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chens::W`](W) writer structure"]
impl crate::Writable for CHENS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHENS to value 0"]
impl crate::Resettable for CHENS_SPEC {
    const RESET_VALUE: u32 = 0;
}
