#[doc = "Register `CHENS` writer"]
pub type W = crate::W<ChensSpec>;
#[doc = "Field `CH0ENS` writer - Channel 0 Enable Set"]
pub type Ch0ensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ENS` writer - Channel 1 Enable Set"]
pub type Ch1ensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ENS` writer - Channel 2 Enable Set"]
pub type Ch2ensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ENS` writer - Channel 3 Enable Set"]
pub type Ch3ensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4ENS` writer - Channel 4 Enable Set"]
pub type Ch4ensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5ENS` writer - Channel 5 Enable Set"]
pub type Ch5ensW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Set"]
    #[inline(always)]
    pub fn ch0ens(&mut self) -> Ch0ensW<'_, ChensSpec> {
        Ch0ensW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Enable Set"]
    #[inline(always)]
    pub fn ch1ens(&mut self) -> Ch1ensW<'_, ChensSpec> {
        Ch1ensW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Enable Set"]
    #[inline(always)]
    pub fn ch2ens(&mut self) -> Ch2ensW<'_, ChensSpec> {
        Ch2ensW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Enable Set"]
    #[inline(always)]
    pub fn ch3ens(&mut self) -> Ch3ensW<'_, ChensSpec> {
        Ch3ensW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Enable Set"]
    #[inline(always)]
    pub fn ch4ens(&mut self) -> Ch4ensW<'_, ChensSpec> {
        Ch4ensW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Enable Set"]
    #[inline(always)]
    pub fn ch5ens(&mut self) -> Ch5ensW<'_, ChensSpec> {
        Ch5ensW::new(self, 5)
    }
}
#[doc = "Channel Enable Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chens::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChensSpec;
impl crate::RegisterSpec for ChensSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chens::W`](W) writer structure"]
impl crate::Writable for ChensSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHENS to value 0"]
impl crate::Resettable for ChensSpec {}
