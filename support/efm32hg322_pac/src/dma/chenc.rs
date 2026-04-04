#[doc = "Register `CHENC` writer"]
pub type W = crate::W<ChencSpec>;
#[doc = "Field `CH0ENC` writer - Channel 0 Enable Clear"]
pub type Ch0encW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ENC` writer - Channel 1 Enable Clear"]
pub type Ch1encW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ENC` writer - Channel 2 Enable Clear"]
pub type Ch2encW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ENC` writer - Channel 3 Enable Clear"]
pub type Ch3encW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4ENC` writer - Channel 4 Enable Clear"]
pub type Ch4encW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5ENC` writer - Channel 5 Enable Clear"]
pub type Ch5encW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Clear"]
    #[inline(always)]
    pub fn ch0enc(&mut self) -> Ch0encW<'_, ChencSpec> {
        Ch0encW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Enable Clear"]
    #[inline(always)]
    pub fn ch1enc(&mut self) -> Ch1encW<'_, ChencSpec> {
        Ch1encW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Enable Clear"]
    #[inline(always)]
    pub fn ch2enc(&mut self) -> Ch2encW<'_, ChencSpec> {
        Ch2encW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Enable Clear"]
    #[inline(always)]
    pub fn ch3enc(&mut self) -> Ch3encW<'_, ChencSpec> {
        Ch3encW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Enable Clear"]
    #[inline(always)]
    pub fn ch4enc(&mut self) -> Ch4encW<'_, ChencSpec> {
        Ch4encW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Enable Clear"]
    #[inline(always)]
    pub fn ch5enc(&mut self) -> Ch5encW<'_, ChencSpec> {
        Ch5encW::new(self, 5)
    }
}
#[doc = "Channel Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChencSpec;
impl crate::RegisterSpec for ChencSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chenc::W`](W) writer structure"]
impl crate::Writable for ChencSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHENC to value 0"]
impl crate::Resettable for ChencSpec {}
