#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Flag"]
pub type CH0DONE_R = crate::BitReader;
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Flag"]
pub type CH1DONE_R = crate::BitReader;
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Flag"]
pub type CH2DONE_R = crate::BitReader;
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Flag"]
pub type CH3DONE_R = crate::BitReader;
#[doc = "Field `CH4DONE` reader - DMA Channel 4 Complete Interrupt Flag"]
pub type CH4DONE_R = crate::BitReader;
#[doc = "Field `CH5DONE` reader - DMA Channel 5 Complete Interrupt Flag"]
pub type CH5DONE_R = crate::BitReader;
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag"]
pub type ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch0done(&self) -> CH0DONE_R {
        CH0DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch1done(&self) -> CH1DONE_R {
        CH1DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch2done(&self) -> CH2DONE_R {
        CH2DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch3done(&self) -> CH3DONE_R {
        CH3DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch4done(&self) -> CH4DONE_R {
        CH4DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch5done(&self) -> CH5DONE_R {
        CH5DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: u32 = 0;
}
