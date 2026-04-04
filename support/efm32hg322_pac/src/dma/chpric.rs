#[doc = "Register `CHPRIC` writer"]
pub type W = crate::W<ChpricSpec>;
#[doc = "Field `CH0PRIC` writer - Channel 0 High Priority Clear"]
pub type Ch0pricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PRIC` writer - Channel 1 High Priority Clear"]
pub type Ch1pricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PRIC` writer - Channel 2 High Priority Clear"]
pub type Ch2pricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PRIC` writer - Channel 3 High Priority Clear"]
pub type Ch3pricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PRIC` writer - Channel 4 High Priority Clear"]
pub type Ch4pricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PRIC` writer - Channel 5 High Priority Clear"]
pub type Ch5pricW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Clear"]
    #[inline(always)]
    pub fn ch0pric(&mut self) -> Ch0pricW<'_, ChpricSpec> {
        Ch0pricW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 High Priority Clear"]
    #[inline(always)]
    pub fn ch1pric(&mut self) -> Ch1pricW<'_, ChpricSpec> {
        Ch1pricW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 High Priority Clear"]
    #[inline(always)]
    pub fn ch2pric(&mut self) -> Ch2pricW<'_, ChpricSpec> {
        Ch2pricW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 High Priority Clear"]
    #[inline(always)]
    pub fn ch3pric(&mut self) -> Ch3pricW<'_, ChpricSpec> {
        Ch3pricW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 High Priority Clear"]
    #[inline(always)]
    pub fn ch4pric(&mut self) -> Ch4pricW<'_, ChpricSpec> {
        Ch4pricW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 High Priority Clear"]
    #[inline(always)]
    pub fn ch5pric(&mut self) -> Ch5pricW<'_, ChpricSpec> {
        Ch5pricW::new(self, 5)
    }
}
#[doc = "Channel Priority Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpric::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChpricSpec;
impl crate::RegisterSpec for ChpricSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chpric::W`](W) writer structure"]
impl crate::Writable for ChpricSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHPRIC to value 0"]
impl crate::Resettable for ChpricSpec {}
