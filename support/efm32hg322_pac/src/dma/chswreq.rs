#[doc = "Register `CHSWREQ` writer"]
pub type W = crate::W<ChswreqSpec>;
#[doc = "Field `CH0SWREQ` writer - Channel 0 Software Request"]
pub type Ch0swreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1SWREQ` writer - Channel 1 Software Request"]
pub type Ch1swreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2SWREQ` writer - Channel 2 Software Request"]
pub type Ch2swreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3SWREQ` writer - Channel 3 Software Request"]
pub type Ch3swreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4SWREQ` writer - Channel 4 Software Request"]
pub type Ch4swreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5SWREQ` writer - Channel 5 Software Request"]
pub type Ch5swreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Software Request"]
    #[inline(always)]
    pub fn ch0swreq(&mut self) -> Ch0swreqW<'_, ChswreqSpec> {
        Ch0swreqW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Request"]
    #[inline(always)]
    pub fn ch1swreq(&mut self) -> Ch1swreqW<'_, ChswreqSpec> {
        Ch1swreqW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Request"]
    #[inline(always)]
    pub fn ch2swreq(&mut self) -> Ch2swreqW<'_, ChswreqSpec> {
        Ch2swreqW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Request"]
    #[inline(always)]
    pub fn ch3swreq(&mut self) -> Ch3swreqW<'_, ChswreqSpec> {
        Ch3swreqW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Request"]
    #[inline(always)]
    pub fn ch4swreq(&mut self) -> Ch4swreqW<'_, ChswreqSpec> {
        Ch4swreqW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Request"]
    #[inline(always)]
    pub fn ch5swreq(&mut self) -> Ch5swreqW<'_, ChswreqSpec> {
        Ch5swreqW::new(self, 5)
    }
}
#[doc = "Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chswreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChswreqSpec;
impl crate::RegisterSpec for ChswreqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chswreq::W`](W) writer structure"]
impl crate::Writable for ChswreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSWREQ to value 0"]
impl crate::Resettable for ChswreqSpec {}
