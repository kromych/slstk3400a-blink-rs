#[doc = "Register `CHREQMASKC` writer"]
pub type W = crate::W<ChreqmaskcSpec>;
#[doc = "Field `CH0REQMASKC` writer - Channel 0 Request Mask Clear"]
pub type Ch0reqmaskcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1REQMASKC` writer - Channel 1 Request Mask Clear"]
pub type Ch1reqmaskcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2REQMASKC` writer - Channel 2 Request Mask Clear"]
pub type Ch2reqmaskcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3REQMASKC` writer - Channel 3 Request Mask Clear"]
pub type Ch3reqmaskcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4REQMASKC` writer - Channel 4 Request Mask Clear"]
pub type Ch4reqmaskcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5REQMASKC` writer - Channel 5 Request Mask Clear"]
pub type Ch5reqmaskcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Request Mask Clear"]
    #[inline(always)]
    pub fn ch0reqmaskc(&mut self) -> Ch0reqmaskcW<'_, ChreqmaskcSpec> {
        Ch0reqmaskcW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Clear"]
    #[inline(always)]
    pub fn ch1reqmaskc(&mut self) -> Ch1reqmaskcW<'_, ChreqmaskcSpec> {
        Ch1reqmaskcW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Clear"]
    #[inline(always)]
    pub fn ch2reqmaskc(&mut self) -> Ch2reqmaskcW<'_, ChreqmaskcSpec> {
        Ch2reqmaskcW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Clear"]
    #[inline(always)]
    pub fn ch3reqmaskc(&mut self) -> Ch3reqmaskcW<'_, ChreqmaskcSpec> {
        Ch3reqmaskcW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Clear"]
    #[inline(always)]
    pub fn ch4reqmaskc(&mut self) -> Ch4reqmaskcW<'_, ChreqmaskcSpec> {
        Ch4reqmaskcW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Clear"]
    #[inline(always)]
    pub fn ch5reqmaskc(&mut self) -> Ch5reqmaskcW<'_, ChreqmaskcSpec> {
        Ch5reqmaskcW::new(self, 5)
    }
}
#[doc = "Channel Request Mask Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chreqmaskc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChreqmaskcSpec;
impl crate::RegisterSpec for ChreqmaskcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chreqmaskc::W`](W) writer structure"]
impl crate::Writable for ChreqmaskcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHREQMASKC to value 0"]
impl crate::Resettable for ChreqmaskcSpec {}
