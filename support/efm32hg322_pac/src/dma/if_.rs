#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Flag"]
pub type Ch0doneR = crate::BitReader;
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Flag"]
pub type Ch1doneR = crate::BitReader;
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Flag"]
pub type Ch2doneR = crate::BitReader;
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Flag"]
pub type Ch3doneR = crate::BitReader;
#[doc = "Field `CH4DONE` reader - DMA Channel 4 Complete Interrupt Flag"]
pub type Ch4doneR = crate::BitReader;
#[doc = "Field `CH5DONE` reader - DMA Channel 5 Complete Interrupt Flag"]
pub type Ch5doneR = crate::BitReader;
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag"]
pub type ErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch0done(&self) -> Ch0doneR {
        Ch0doneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch1done(&self) -> Ch1doneR {
        Ch1doneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch2done(&self) -> Ch2doneR {
        Ch2doneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch3done(&self) -> Ch3doneR {
        Ch3doneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch4done(&self) -> Ch4doneR {
        Ch4doneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch5done(&self) -> Ch5doneR {
        Ch5doneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
