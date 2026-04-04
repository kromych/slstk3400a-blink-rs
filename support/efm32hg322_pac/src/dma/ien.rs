#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Enable"]
pub type Ch0doneR = crate::BitReader;
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Enable"]
pub type Ch0doneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Enable"]
pub type Ch1doneR = crate::BitReader;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Enable"]
pub type Ch1doneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Enable"]
pub type Ch2doneR = crate::BitReader;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Enable"]
pub type Ch2doneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Enable"]
pub type Ch3doneR = crate::BitReader;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Enable"]
pub type Ch3doneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4DONE` reader - DMA Channel 4 Complete Interrupt Enable"]
pub type Ch4doneR = crate::BitReader;
#[doc = "Field `CH4DONE` writer - DMA Channel 4 Complete Interrupt Enable"]
pub type Ch4doneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5DONE` reader - DMA Channel 5 Complete Interrupt Enable"]
pub type Ch5doneR = crate::BitReader;
#[doc = "Field `CH5DONE` writer - DMA Channel 5 Complete Interrupt Enable"]
pub type Ch5doneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag Enable"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Enable"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&self) -> Ch0doneR {
        Ch0doneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&self) -> Ch1doneR {
        Ch1doneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&self) -> Ch2doneR {
        Ch2doneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&self) -> Ch3doneR {
        Ch3doneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch4done(&self) -> Ch4doneR {
        Ch4doneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch5done(&self) -> Ch5doneR {
        Ch5doneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&mut self) -> Ch0doneW<'_, IenSpec> {
        Ch0doneW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&mut self) -> Ch1doneW<'_, IenSpec> {
        Ch1doneW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&mut self) -> Ch2doneW<'_, IenSpec> {
        Ch2doneW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&mut self) -> Ch3doneW<'_, IenSpec> {
        Ch3doneW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch4done(&mut self) -> Ch4doneW<'_, IenSpec> {
        Ch4doneW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch5done(&mut self) -> Ch5doneW<'_, IenSpec> {
        Ch5doneW::new(self, 5)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IenSpec> {
        ErrW::new(self, 31)
    }
}
#[doc = "Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
