#[doc = "Register `CHALTC` writer"]
pub type W = crate::W<ChaltcSpec>;
#[doc = "Field `CH0ALTC` writer - Channel 0 Alternate Clear"]
pub type Ch0altcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ALTC` writer - Channel 1 Alternate Clear"]
pub type Ch1altcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ALTC` writer - Channel 2 Alternate Clear"]
pub type Ch2altcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ALTC` writer - Channel 3 Alternate Clear"]
pub type Ch3altcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4ALTC` writer - Channel 4 Alternate Clear"]
pub type Ch4altcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5ALTC` writer - Channel 5 Alternate Clear"]
pub type Ch5altcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Clear"]
    #[inline(always)]
    pub fn ch0altc(&mut self) -> Ch0altcW<'_, ChaltcSpec> {
        Ch0altcW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Clear"]
    #[inline(always)]
    pub fn ch1altc(&mut self) -> Ch1altcW<'_, ChaltcSpec> {
        Ch1altcW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Clear"]
    #[inline(always)]
    pub fn ch2altc(&mut self) -> Ch2altcW<'_, ChaltcSpec> {
        Ch2altcW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Clear"]
    #[inline(always)]
    pub fn ch3altc(&mut self) -> Ch3altcW<'_, ChaltcSpec> {
        Ch3altcW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Alternate Clear"]
    #[inline(always)]
    pub fn ch4altc(&mut self) -> Ch4altcW<'_, ChaltcSpec> {
        Ch4altcW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Alternate Clear"]
    #[inline(always)]
    pub fn ch5altc(&mut self) -> Ch5altcW<'_, ChaltcSpec> {
        Ch5altcW::new(self, 5)
    }
}
#[doc = "Channel Alternate Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chaltc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChaltcSpec;
impl crate::RegisterSpec for ChaltcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chaltc::W`](W) writer structure"]
impl crate::Writable for ChaltcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHALTC to value 0"]
impl crate::Resettable for ChaltcSpec {}
