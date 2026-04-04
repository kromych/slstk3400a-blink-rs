#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `TXC` writer - Clear TX Complete Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Clear RX Overflow Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Clear RX Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Clear TX Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Clear Parity Error Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Clear Framing Error Interrupt Flag"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Clear Multi-Processor Address Frame Interrupt Flag"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTF` writer - Clear Start-Frame Interrupt Flag"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGF` writer - Clear Signal-Frame Interrupt Flag"]
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfcSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 3 - Clear RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IfcSpec> {
        RxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfcSpec> {
        RxufW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfcSpec> {
        TxofW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IfcSpec> {
        PerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IfcSpec> {
        FerrW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IfcSpec> {
        MpafW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Start-Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<'_, IfcSpec> {
        StartfW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Signal-Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SigfW<'_, IfcSpec> {
        SigfW::new(self, 10)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
