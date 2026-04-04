#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `TXC` reader - TX Complete Interrupt Flag"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Flag"]
pub type TxblR = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Flag"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Flag"]
pub type RxofR = crate::BitReader;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Flag"]
pub type RxufR = crate::BitReader;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Flag"]
pub type TxofR = crate::BitReader;
#[doc = "Field `PERR` reader - Parity Error Interrupt Flag"]
pub type PerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Framing Error Interrupt Flag"]
pub type FerrR = crate::BitReader;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt Flag"]
pub type MpafR = crate::BitReader;
#[doc = "Field `STARTF` reader - Start Frame Interrupt Flag"]
pub type StartfR = crate::BitReader;
#[doc = "Field `SIGF` reader - Signal Frame Interrupt Flag"]
pub type SigfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Signal Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&self) -> SigfR {
        SigfR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0x02"]
impl crate::Resettable for IfSpec {
    const RESET_VALUE: u32 = 0x02;
}
