#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `TXC` writer - Clear TX Complete Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Clear RX Buffer Full Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Clear RX Overflow Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Clear RX Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Clear TX Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` writer - Clear TX Underflow Interrupt Flag"]
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Clear Parity Error Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Clear Framing Error Interrupt Flag"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Clear Multi-Processor Address Frame Interrupt Flag"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` writer - Clear Slave-Select In Master Mode Interrupt Flag"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` writer - Clear Collision Check Fail Interrupt Flag"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfcSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 3 - Clear RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IfcSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IfcSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfcSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfcSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear TX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TxufW<'_, IfcSpec> {
        TxufW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IfcSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IfcSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IfcSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Slave-Select In Master Mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<'_, IfcSpec> {
        SsmW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<'_, IfcSpec> {
        CcfW::new(self, 12)
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
