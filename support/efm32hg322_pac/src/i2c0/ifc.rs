#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `START` writer - Clear START Interrupt Flag"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` writer - Clear Repeated START Interrupt Flag"]
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` writer - Clear Address Interrupt Flag"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` writer - Clear Transfer Completed Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` writer - Clear Acknowledge Received Interrupt Flag"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Clear Not Acknowledge Received Interrupt Flag"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` writer - Clear MSTOP Interrupt Flag"]
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` writer - Clear Arbitration Lost Interrupt Flag"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` writer - Clear Bus Error Interrupt Flag"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` writer - Clear Bus Held Interrupt Flag"]
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Clear Transmit Buffer Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Clear Receive Buffer Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` writer - Clear Bus Idle Timeout Interrupt Flag"]
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` writer - Clear Clock Low Interrupt Flag"]
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` writer - Clear SSTOP Interrupt Flag"]
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear START Interrupt Flag"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IfcSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Repeated START Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RstartW<'_, IfcSpec> {
        RstartW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, IfcSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfcSpec> {
        TxcW::new(self, 3)
    }
    #[doc = "Bit 6 - Clear Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, IfcSpec> {
        AckW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, IfcSpec> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear MSTOP Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MstopW<'_, IfcSpec> {
        MstopW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, IfcSpec> {
        ArblostW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, IfcSpec> {
        BuserrW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BusholdW<'_, IfcSpec> {
        BusholdW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfcSpec> {
        TxofW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfcSpec> {
        RxufW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&mut self) -> BitoW<'_, IfcSpec> {
        BitoW::new(self, 14)
    }
    #[doc = "Bit 15 - Clear Clock Low Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&mut self) -> CltoW<'_, IfcSpec> {
        CltoW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear SSTOP Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SstopW<'_, IfcSpec> {
        SstopW::new(self, 16)
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
