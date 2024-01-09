#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Flag Clear"]
pub type CH0DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Flag Clear"]
pub type CH1DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Flag Clear"]
pub type CH2DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Flag Clear"]
pub type CH3DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4DONE` writer - DMA Channel 4 Complete Interrupt Flag Clear"]
pub type CH4DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5DONE` writer - DMA Channel 5 Complete Interrupt Flag Clear"]
pub type CH5DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Clear"]
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0done(&mut self) -> CH0DONE_W<IFC_SPEC> {
        CH0DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1done(&mut self) -> CH1DONE_W<IFC_SPEC> {
        CH1DONE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2done(&mut self) -> CH2DONE_W<IFC_SPEC> {
        CH2DONE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3done(&mut self) -> CH3DONE_W<IFC_SPEC> {
        CH3DONE_W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch4done(&mut self) -> CH4DONE_W<IFC_SPEC> {
        CH4DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch5done(&mut self) -> CH5DONE_W<IFC_SPEC> {
        CH5DONE_W::new(self, 5)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<IFC_SPEC> {
        ERR_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
