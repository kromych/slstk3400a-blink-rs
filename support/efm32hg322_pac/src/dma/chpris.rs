#[doc = "Register `CHPRIS` writer"]
pub type W = crate::W<ChprisSpec>;
#[doc = "Field `CH0PRIS` writer - Channel 0 High Priority Set"]
pub type Ch0prisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PRIS` writer - Channel 1 High Priority Set"]
pub type Ch1prisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PRIS` writer - Channel 2 High Priority Set"]
pub type Ch2prisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PRIS` writer - Channel 3 High Priority Set"]
pub type Ch3prisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PRIS` writer - Channel 4 High Priority Set"]
pub type Ch4prisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PRIS` writer - Channel 5 High Priority Set"]
pub type Ch5prisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Set"]
    #[inline(always)]
    pub fn ch0pris(&mut self) -> Ch0prisW<'_, ChprisSpec> {
        Ch0prisW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 High Priority Set"]
    #[inline(always)]
    pub fn ch1pris(&mut self) -> Ch1prisW<'_, ChprisSpec> {
        Ch1prisW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 High Priority Set"]
    #[inline(always)]
    pub fn ch2pris(&mut self) -> Ch2prisW<'_, ChprisSpec> {
        Ch2prisW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 High Priority Set"]
    #[inline(always)]
    pub fn ch3pris(&mut self) -> Ch3prisW<'_, ChprisSpec> {
        Ch3prisW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 High Priority Set"]
    #[inline(always)]
    pub fn ch4pris(&mut self) -> Ch4prisW<'_, ChprisSpec> {
        Ch4prisW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 High Priority Set"]
    #[inline(always)]
    pub fn ch5pris(&mut self) -> Ch5prisW<'_, ChprisSpec> {
        Ch5prisW::new(self, 5)
    }
}
#[doc = "Channel Priority Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpris::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChprisSpec;
impl crate::RegisterSpec for ChprisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chpris::W`](W) writer structure"]
impl crate::Writable for ChprisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHPRIS to value 0"]
impl crate::Resettable for ChprisSpec {}
