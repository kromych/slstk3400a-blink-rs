#[doc = "Register `CHALTS` writer"]
pub type W = crate::W<ChaltsSpec>;
#[doc = "Field `CH0ALTS` writer - Channel 0 Alternate Structure Set"]
pub type Ch0altsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ALTS` writer - Channel 1 Alternate Structure Set"]
pub type Ch1altsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ALTS` writer - Channel 2 Alternate Structure Set"]
pub type Ch2altsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ALTS` writer - Channel 3 Alternate Structure Set"]
pub type Ch3altsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4ALTS` writer - Channel 4 Alternate Structure Set"]
pub type Ch4altsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5ALTS` writer - Channel 5 Alternate Structure Set"]
pub type Ch5altsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch0alts(&mut self) -> Ch0altsW<'_, ChaltsSpec> {
        Ch0altsW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch1alts(&mut self) -> Ch1altsW<'_, ChaltsSpec> {
        Ch1altsW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch2alts(&mut self) -> Ch2altsW<'_, ChaltsSpec> {
        Ch2altsW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch3alts(&mut self) -> Ch3altsW<'_, ChaltsSpec> {
        Ch3altsW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch4alts(&mut self) -> Ch4altsW<'_, ChaltsSpec> {
        Ch4altsW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch5alts(&mut self) -> Ch5altsW<'_, ChaltsSpec> {
        Ch5altsW::new(self, 5)
    }
}
#[doc = "Channel Alternate Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chalts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChaltsSpec;
impl crate::RegisterSpec for ChaltsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chalts::W`](W) writer structure"]
impl crate::Writable for ChaltsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHALTS to value 0"]
impl crate::Resettable for ChaltsSpec {}
