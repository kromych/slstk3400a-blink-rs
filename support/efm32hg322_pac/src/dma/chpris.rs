#[doc = "Register `CHPRIS` writer"]
pub type W = crate::W<CHPRIS_SPEC>;
#[doc = "Field `CH0PRIS` writer - Channel 0 High Priority Set"]
pub type CH0PRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1PRIS` writer - Channel 1 High Priority Set"]
pub type CH1PRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2PRIS` writer - Channel 2 High Priority Set"]
pub type CH2PRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3PRIS` writer - Channel 3 High Priority Set"]
pub type CH3PRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4PRIS` writer - Channel 4 High Priority Set"]
pub type CH4PRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5PRIS` writer - Channel 5 High Priority Set"]
pub type CH5PRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pris(&mut self) -> CH0PRIS_W<CHPRIS_SPEC, 0> {
        CH0PRIS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 High Priority Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pris(&mut self) -> CH1PRIS_W<CHPRIS_SPEC, 1> {
        CH1PRIS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 High Priority Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pris(&mut self) -> CH2PRIS_W<CHPRIS_SPEC, 2> {
        CH2PRIS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 High Priority Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pris(&mut self) -> CH3PRIS_W<CHPRIS_SPEC, 3> {
        CH3PRIS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 High Priority Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pris(&mut self) -> CH4PRIS_W<CHPRIS_SPEC, 4> {
        CH4PRIS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 High Priority Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pris(&mut self) -> CH5PRIS_W<CHPRIS_SPEC, 5> {
        CH5PRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Priority Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpris::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHPRIS_SPEC;
impl crate::RegisterSpec for CHPRIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chpris::W`](W) writer structure"]
impl crate::Writable for CHPRIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHPRIS to value 0"]
impl crate::Resettable for CHPRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
