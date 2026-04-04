#[doc = "Register `CHUSEBURSTC` writer"]
pub type W = crate::W<ChuseburstcSpec>;
#[doc = "Field `CH0USEBURSTC` writer - Channel 0 Useburst Clear"]
pub type Ch0useburstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1USEBURSTC` writer - Channel 1 Useburst Clear"]
pub type Ch1useburstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2USEBURSTC` writer - Channel 2 Useburst Clear"]
pub type Ch2useburstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3USEBURSTC` writer - Channel 3 Useburst Clear"]
pub type Ch3useburstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4USEBURSTC` writer - Channel 4 Useburst Clear"]
pub type Ch4useburstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5USEBURSTC` writer - Channel 5 Useburst Clear"]
pub type Ch5useburstcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Clear"]
    #[inline(always)]
    pub fn ch0useburstc(&mut self) -> Ch0useburstcW<'_, ChuseburstcSpec> {
        Ch0useburstcW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Clear"]
    #[inline(always)]
    pub fn ch1useburstc(&mut self) -> Ch1useburstcW<'_, ChuseburstcSpec> {
        Ch1useburstcW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Clear"]
    #[inline(always)]
    pub fn ch2useburstc(&mut self) -> Ch2useburstcW<'_, ChuseburstcSpec> {
        Ch2useburstcW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Clear"]
    #[inline(always)]
    pub fn ch3useburstc(&mut self) -> Ch3useburstcW<'_, ChuseburstcSpec> {
        Ch3useburstcW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Clear"]
    #[inline(always)]
    pub fn ch4useburstc(&mut self) -> Ch4useburstcW<'_, ChuseburstcSpec> {
        Ch4useburstcW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Clear"]
    #[inline(always)]
    pub fn ch5useburstc(&mut self) -> Ch5useburstcW<'_, ChuseburstcSpec> {
        Ch5useburstcW::new(self, 5)
    }
}
#[doc = "Channel Useburst Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chuseburstc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChuseburstcSpec;
impl crate::RegisterSpec for ChuseburstcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chuseburstc::W`](W) writer structure"]
impl crate::Writable for ChuseburstcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHUSEBURSTC to value 0"]
impl crate::Resettable for ChuseburstcSpec {}
