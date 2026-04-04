#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `START` reader - START Condition Interrupt Enable"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START Condition Interrupt Enable"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` reader - Repeated START condition Interrupt Enable"]
pub type RstartR = crate::BitReader;
#[doc = "Field `RSTART` writer - Repeated START condition Interrupt Enable"]
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Address Interrupt Enable"]
pub type AddrR = crate::BitReader;
#[doc = "Field `ADDR` writer - Address Interrupt Enable"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - Transfer Completed Interrupt Enable"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transfer Completed Interrupt Enable"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBL` reader - Transmit Buffer level Interrupt Enable"]
pub type TxblR = crate::BitReader;
#[doc = "Field `TXBL` writer - Transmit Buffer level Interrupt Enable"]
pub type TxblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDATAV` reader - Receive Data Valid Interrupt Enable"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXDATAV` writer - Receive Data Valid Interrupt Enable"]
pub type RxdatavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledge Received Interrupt Enable"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge Received Interrupt Enable"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Not Acknowledge Received Interrupt Enable"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - Not Acknowledge Received Interrupt Enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` reader - MSTOP Interrupt Enable"]
pub type MstopR = crate::BitReader;
#[doc = "Field `MSTOP` writer - MSTOP Interrupt Enable"]
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt Enable"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `ARBLOST` writer - Arbitration Lost Interrupt Enable"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` reader - Bus Error Interrupt Enable"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus Error Interrupt Enable"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` reader - Bus Held Interrupt Enable"]
pub type BusholdR = crate::BitReader;
#[doc = "Field `BUSHOLD` writer - Bus Held Interrupt Enable"]
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - Transmit Buffer Overflow Interrupt Enable"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXOF` writer - Transmit Buffer Overflow Interrupt Enable"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - Receive Buffer Underflow Interrupt Enable"]
pub type RxufR = crate::BitReader;
#[doc = "Field `RXUF` writer - Receive Buffer Underflow Interrupt Enable"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` reader - Bus Idle Timeout Interrupt Enable"]
pub type BitoR = crate::BitReader;
#[doc = "Field `BITO` writer - Bus Idle Timeout Interrupt Enable"]
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` reader - Clock Low Interrupt Enable"]
pub type CltoR = crate::BitReader;
#[doc = "Field `CLTO` writer - Clock Low Interrupt Enable"]
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` reader - SSTOP Interrupt Enable"]
pub type SstopR = crate::BitReader;
#[doc = "Field `SSTOP` writer - SSTOP Interrupt Enable"]
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - START Condition Interrupt Enable"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Enable"]
    #[inline(always)]
    pub fn rstart(&self) -> RstartR {
        RstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Interrupt Enable"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Buffer level Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn mstop(&self) -> MstopR {
        MstopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Enable"]
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Low Interrupt Enable"]
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn sstop(&self) -> SstopR {
        SstopR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START Condition Interrupt Enable"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IenSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Enable"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RstartW<'_, IenSpec> {
        RstartW::new(self, 1)
    }
    #[doc = "Bit 2 - Address Interrupt Enable"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, IenSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IenSpec> {
        TxcW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Buffer level Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TxblW<'_, IenSpec> {
        TxblW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RxdatavW<'_, IenSpec> {
        RxdatavW::new(self, 5)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, IenSpec> {
        AckW::new(self, 6)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, IenSpec> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MstopW<'_, IenSpec> {
        MstopW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, IenSpec> {
        ArblostW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, IenSpec> {
        BuserrW::new(self, 10)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Enable"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BusholdW<'_, IenSpec> {
        BusholdW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IenSpec> {
        TxofW::new(self, 12)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IenSpec> {
        RxufW::new(self, 13)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn bito(&mut self) -> BitoW<'_, IenSpec> {
        BitoW::new(self, 14)
    }
    #[doc = "Bit 15 - Clock Low Interrupt Enable"]
    #[inline(always)]
    pub fn clto(&mut self) -> CltoW<'_, IenSpec> {
        CltoW::new(self, 15)
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SstopW<'_, IenSpec> {
        SstopW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
