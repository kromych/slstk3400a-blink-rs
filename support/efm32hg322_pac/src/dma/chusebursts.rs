#[doc = "Register `CHUSEBURSTS` reader"]
pub type R = crate::R<ChuseburstsSpec>;
#[doc = "Register `CHUSEBURSTS` writer"]
pub type W = crate::W<ChuseburstsSpec>;
#[doc = "Field `CH0USEBURSTS` reader - Channel 0 Useburst Set"]
pub type Ch0useburstsR = crate::BitReader;
#[doc = "Field `CH0USEBURSTS` writer - Channel 0 Useburst Set"]
pub type Ch0useburstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1USEBURSTS` reader - Channel 1 Useburst Set"]
pub type Ch1useburstsR = crate::BitReader;
#[doc = "Field `CH1USEBURSTS` writer - Channel 1 Useburst Set"]
pub type Ch1useburstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2USEBURSTS` reader - Channel 2 Useburst Set"]
pub type Ch2useburstsR = crate::BitReader;
#[doc = "Field `CH2USEBURSTS` writer - Channel 2 Useburst Set"]
pub type Ch2useburstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3USEBURSTS` reader - Channel 3 Useburst Set"]
pub type Ch3useburstsR = crate::BitReader;
#[doc = "Field `CH3USEBURSTS` writer - Channel 3 Useburst Set"]
pub type Ch3useburstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4USEBURSTS` reader - Channel 4 Useburst Set"]
pub type Ch4useburstsR = crate::BitReader;
#[doc = "Field `CH4USEBURSTS` writer - Channel 4 Useburst Set"]
pub type Ch4useburstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5USEBURSTS` reader - Channel 5 Useburst Set"]
pub type Ch5useburstsR = crate::BitReader;
#[doc = "Field `CH5USEBURSTS` writer - Channel 5 Useburst Set"]
pub type Ch5useburstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    pub fn ch0usebursts(&self) -> Ch0useburstsR {
        Ch0useburstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    pub fn ch1usebursts(&self) -> Ch1useburstsR {
        Ch1useburstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    pub fn ch2usebursts(&self) -> Ch2useburstsR {
        Ch2useburstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    pub fn ch3usebursts(&self) -> Ch3useburstsR {
        Ch3useburstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    pub fn ch4usebursts(&self) -> Ch4useburstsR {
        Ch4useburstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    pub fn ch5usebursts(&self) -> Ch5useburstsR {
        Ch5useburstsR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    pub fn ch0usebursts(&mut self) -> Ch0useburstsW<'_, ChuseburstsSpec> {
        Ch0useburstsW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    pub fn ch1usebursts(&mut self) -> Ch1useburstsW<'_, ChuseburstsSpec> {
        Ch1useburstsW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    pub fn ch2usebursts(&mut self) -> Ch2useburstsW<'_, ChuseburstsSpec> {
        Ch2useburstsW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    pub fn ch3usebursts(&mut self) -> Ch3useburstsW<'_, ChuseburstsSpec> {
        Ch3useburstsW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    pub fn ch4usebursts(&mut self) -> Ch4useburstsW<'_, ChuseburstsSpec> {
        Ch4useburstsW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    pub fn ch5usebursts(&mut self) -> Ch5useburstsW<'_, ChuseburstsSpec> {
        Ch5useburstsW::new(self, 5)
    }
}
#[doc = "Channel Useburst Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chusebursts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chusebursts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChuseburstsSpec;
impl crate::RegisterSpec for ChuseburstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chusebursts::R`](R) reader structure"]
impl crate::Readable for ChuseburstsSpec {}
#[doc = "`write(|w| ..)` method takes [`chusebursts::W`](W) writer structure"]
impl crate::Writable for ChuseburstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHUSEBURSTS to value 0"]
impl crate::Resettable for ChuseburstsSpec {}
