#[doc = "Register `CHSWREQ` writer"]
pub type W = crate::W<CHSWREQ_SPEC>;
#[doc = "Field `CH0SWREQ` writer - Channel 0 Software Request"]
pub type CH0SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1SWREQ` writer - Channel 1 Software Request"]
pub type CH1SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2SWREQ` writer - Channel 2 Software Request"]
pub type CH2SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3SWREQ` writer - Channel 3 Software Request"]
pub type CH3SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4SWREQ` writer - Channel 4 Software Request"]
pub type CH4SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5SWREQ` writer - Channel 5 Software Request"]
pub type CH5SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Software Request"]
    #[inline(always)]
    #[must_use]
    pub fn ch0swreq(&mut self) -> CH0SWREQ_W<CHSWREQ_SPEC> {
        CH0SWREQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Request"]
    #[inline(always)]
    #[must_use]
    pub fn ch1swreq(&mut self) -> CH1SWREQ_W<CHSWREQ_SPEC> {
        CH1SWREQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Request"]
    #[inline(always)]
    #[must_use]
    pub fn ch2swreq(&mut self) -> CH2SWREQ_W<CHSWREQ_SPEC> {
        CH2SWREQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Request"]
    #[inline(always)]
    #[must_use]
    pub fn ch3swreq(&mut self) -> CH3SWREQ_W<CHSWREQ_SPEC> {
        CH3SWREQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Request"]
    #[inline(always)]
    #[must_use]
    pub fn ch4swreq(&mut self) -> CH4SWREQ_W<CHSWREQ_SPEC> {
        CH4SWREQ_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Request"]
    #[inline(always)]
    #[must_use]
    pub fn ch5swreq(&mut self) -> CH5SWREQ_W<CHSWREQ_SPEC> {
        CH5SWREQ_W::new(self, 5)
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
#[doc = "Channel Software Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chswreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSWREQ_SPEC;
impl crate::RegisterSpec for CHSWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chswreq::W`](W) writer structure"]
impl crate::Writable for CHSWREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSWREQ to value 0"]
impl crate::Resettable for CHSWREQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
