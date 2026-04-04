#[doc = "Register `SWLEVEL` reader"]
pub type R = crate::R<SwlevelSpec>;
#[doc = "Register `SWLEVEL` writer"]
pub type W = crate::W<SwlevelSpec>;
#[doc = "Field `CH0LEVEL` reader - Channel 0 Software Level"]
pub type Ch0levelR = crate::BitReader;
#[doc = "Field `CH0LEVEL` writer - Channel 0 Software Level"]
pub type Ch0levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1LEVEL` reader - Channel 1 Software Level"]
pub type Ch1levelR = crate::BitReader;
#[doc = "Field `CH1LEVEL` writer - Channel 1 Software Level"]
pub type Ch1levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2LEVEL` reader - Channel 2 Software Level"]
pub type Ch2levelR = crate::BitReader;
#[doc = "Field `CH2LEVEL` writer - Channel 2 Software Level"]
pub type Ch2levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3LEVEL` reader - Channel 3 Software Level"]
pub type Ch3levelR = crate::BitReader;
#[doc = "Field `CH3LEVEL` writer - Channel 3 Software Level"]
pub type Ch3levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4LEVEL` reader - Channel 4 Software Level"]
pub type Ch4levelR = crate::BitReader;
#[doc = "Field `CH4LEVEL` writer - Channel 4 Software Level"]
pub type Ch4levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5LEVEL` reader - Channel 5 Software Level"]
pub type Ch5levelR = crate::BitReader;
#[doc = "Field `CH5LEVEL` writer - Channel 5 Software Level"]
pub type Ch5levelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> Ch0levelR {
        Ch0levelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> Ch1levelR {
        Ch1levelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> Ch2levelR {
        Ch2levelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> Ch3levelR {
        Ch3levelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> Ch4levelR {
        Ch4levelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> Ch5levelR {
        Ch5levelR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&mut self) -> Ch0levelW<'_, SwlevelSpec> {
        Ch0levelW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&mut self) -> Ch1levelW<'_, SwlevelSpec> {
        Ch1levelW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&mut self) -> Ch2levelW<'_, SwlevelSpec> {
        Ch2levelW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&mut self) -> Ch3levelW<'_, SwlevelSpec> {
        Ch3levelW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&mut self) -> Ch4levelW<'_, SwlevelSpec> {
        Ch4levelW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&mut self) -> Ch5levelW<'_, SwlevelSpec> {
        Ch5levelW::new(self, 5)
    }
}
#[doc = "Software Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swlevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swlevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwlevelSpec;
impl crate::RegisterSpec for SwlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swlevel::R`](R) reader structure"]
impl crate::Readable for SwlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`swlevel::W`](W) writer structure"]
impl crate::Writable for SwlevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWLEVEL to value 0"]
impl crate::Resettable for SwlevelSpec {}
