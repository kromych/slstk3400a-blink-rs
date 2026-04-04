#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `START` reader - START condition Interrupt Flag"]
pub type StartR = crate::BitReader;
#[doc = "Field `RSTART` reader - Repeated START condition Interrupt Flag"]
pub type RstartR = crate::BitReader;
#[doc = "Field `ADDR` reader - Address Interrupt Flag"]
pub type AddrR = crate::BitReader;
#[doc = "Field `TXC` reader - Transfer Completed Interrupt Flag"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXBL` reader - Transmit Buffer Level Interrupt Flag"]
pub type TxblR = crate::BitReader;
#[doc = "Field `RXDATAV` reader - Receive Data Valid Interrupt Flag"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `ACK` reader - Acknowledge Received Interrupt Flag"]
pub type AckR = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledge Received Interrupt Flag"]
pub type NackR = crate::BitReader;
#[doc = "Field `MSTOP` reader - Master STOP Condition Interrupt Flag"]
pub type MstopR = crate::BitReader;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt Flag"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `BUSERR` reader - Bus Error Interrupt Flag"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `BUSHOLD` reader - Bus Held Interrupt Flag"]
pub type BusholdR = crate::BitReader;
#[doc = "Field `TXOF` reader - Transmit Buffer Overflow Interrupt Flag"]
pub type TxofR = crate::BitReader;
#[doc = "Field `RXUF` reader - Receive Buffer Underflow Interrupt Flag"]
pub type RxufR = crate::BitReader;
#[doc = "Field `BITO` reader - Bus Idle Timeout Interrupt Flag"]
pub type BitoR = crate::BitReader;
#[doc = "Field `CLTO` reader - Clock Low Timeout Interrupt Flag"]
pub type CltoR = crate::BitReader;
#[doc = "Field `SSTOP` reader - Slave STOP condition Interrupt Flag"]
pub type SstopR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - START condition Interrupt Flag"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&self) -> RstartR {
        RstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&self) -> MstopR {
        MstopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Low Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave STOP condition Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&self) -> SstopR {
        SstopR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0x10"]
impl crate::Resettable for IfSpec {
    const RESET_VALUE: u32 = 0x10;
}
