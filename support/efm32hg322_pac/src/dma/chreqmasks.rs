#[doc = "Register `CHREQMASKS` writer"]
pub type W = crate::W<ChreqmasksSpec>;
#[doc = "Field `CH0REQMASKS` writer - Channel 0 Request Mask Set"]
pub type Ch0reqmasksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1REQMASKS` writer - Channel 1 Request Mask Set"]
pub type Ch1reqmasksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2REQMASKS` writer - Channel 2 Request Mask Set"]
pub type Ch2reqmasksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3REQMASKS` writer - Channel 3 Request Mask Set"]
pub type Ch3reqmasksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4REQMASKS` writer - Channel 4 Request Mask Set"]
pub type Ch4reqmasksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5REQMASKS` writer - Channel 5 Request Mask Set"]
pub type Ch5reqmasksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Request Mask Set"]
    #[inline(always)]
    pub fn ch0reqmasks(&mut self) -> Ch0reqmasksW<'_, ChreqmasksSpec> {
        Ch0reqmasksW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Set"]
    #[inline(always)]
    pub fn ch1reqmasks(&mut self) -> Ch1reqmasksW<'_, ChreqmasksSpec> {
        Ch1reqmasksW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Set"]
    #[inline(always)]
    pub fn ch2reqmasks(&mut self) -> Ch2reqmasksW<'_, ChreqmasksSpec> {
        Ch2reqmasksW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Set"]
    #[inline(always)]
    pub fn ch3reqmasks(&mut self) -> Ch3reqmasksW<'_, ChreqmasksSpec> {
        Ch3reqmasksW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Set"]
    #[inline(always)]
    pub fn ch4reqmasks(&mut self) -> Ch4reqmasksW<'_, ChreqmasksSpec> {
        Ch4reqmasksW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Set"]
    #[inline(always)]
    pub fn ch5reqmasks(&mut self) -> Ch5reqmasksW<'_, ChreqmasksSpec> {
        Ch5reqmasksW::new(self, 5)
    }
}
#[doc = "Channel Request Mask Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chreqmasks::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChreqmasksSpec;
impl crate::RegisterSpec for ChreqmasksSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chreqmasks::W`](W) writer structure"]
impl crate::Writable for ChreqmasksSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHREQMASKS to value 0"]
impl crate::Resettable for ChreqmasksSpec {}
