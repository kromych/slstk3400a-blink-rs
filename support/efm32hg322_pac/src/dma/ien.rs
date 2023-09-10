#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Enable"]
pub type CH0DONE_R = crate::BitReader;
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Enable"]
pub type CH0DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Enable"]
pub type CH1DONE_R = crate::BitReader;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Enable"]
pub type CH1DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Enable"]
pub type CH2DONE_R = crate::BitReader;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Enable"]
pub type CH2DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Enable"]
pub type CH3DONE_R = crate::BitReader;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Enable"]
pub type CH3DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4DONE` reader - DMA Channel 4 Complete Interrupt Enable"]
pub type CH4DONE_R = crate::BitReader;
#[doc = "Field `CH4DONE` writer - DMA Channel 4 Complete Interrupt Enable"]
pub type CH4DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5DONE` reader - DMA Channel 5 Complete Interrupt Enable"]
pub type CH5DONE_R = crate::BitReader;
#[doc = "Field `CH5DONE` writer - DMA Channel 5 Complete Interrupt Enable"]
pub type CH5DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag Enable"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Enable"]
pub type ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&self) -> CH0DONE_R {
        CH0DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&self) -> CH1DONE_R {
        CH1DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&self) -> CH2DONE_R {
        CH2DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&self) -> CH3DONE_R {
        CH3DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch4done(&self) -> CH4DONE_R {
        CH4DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch5done(&self) -> CH5DONE_R {
        CH5DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0done(&mut self) -> CH0DONE_W<IEN_SPEC, 0> {
        CH0DONE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1done(&mut self) -> CH1DONE_W<IEN_SPEC, 1> {
        CH1DONE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2done(&mut self) -> CH2DONE_W<IEN_SPEC, 2> {
        CH2DONE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3done(&mut self) -> CH3DONE_W<IEN_SPEC, 3> {
        CH3DONE_W::new(self)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4done(&mut self) -> CH4DONE_W<IEN_SPEC, 4> {
        CH4DONE_W::new(self)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5done(&mut self) -> CH5DONE_W<IEN_SPEC, 5> {
        CH5DONE_W::new(self)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<IEN_SPEC, 31> {
        ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
