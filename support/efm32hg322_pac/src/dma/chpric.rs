#[doc = "Register `CHPRIC` writer"]
pub type W = crate::W<CHPRIC_SPEC>;
#[doc = "Field `CH0PRIC` writer - Channel 0 High Priority Clear"]
pub type CH0PRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1PRIC` writer - Channel 1 High Priority Clear"]
pub type CH1PRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2PRIC` writer - Channel 2 High Priority Clear"]
pub type CH2PRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3PRIC` writer - Channel 3 High Priority Clear"]
pub type CH3PRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4PRIC` writer - Channel 4 High Priority Clear"]
pub type CH4PRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5PRIC` writer - Channel 5 High Priority Clear"]
pub type CH5PRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pric(&mut self) -> CH0PRIC_W<CHPRIC_SPEC, 0> {
        CH0PRIC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pric(&mut self) -> CH1PRIC_W<CHPRIC_SPEC, 1> {
        CH1PRIC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pric(&mut self) -> CH2PRIC_W<CHPRIC_SPEC, 2> {
        CH2PRIC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pric(&mut self) -> CH3PRIC_W<CHPRIC_SPEC, 3> {
        CH3PRIC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pric(&mut self) -> CH4PRIC_W<CHPRIC_SPEC, 4> {
        CH4PRIC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 High Priority Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pric(&mut self) -> CH5PRIC_W<CHPRIC_SPEC, 5> {
        CH5PRIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Priority Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpric::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHPRIC_SPEC;
impl crate::RegisterSpec for CHPRIC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chpric::W`](W) writer structure"]
impl crate::Writable for CHPRIC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHPRIC to value 0"]
impl crate::Resettable for CHPRIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
