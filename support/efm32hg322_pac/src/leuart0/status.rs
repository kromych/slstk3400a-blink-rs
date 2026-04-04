#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RXENS` reader - Receiver Enable Status"]
pub type RxensR = crate::BitReader;
#[doc = "Field `TXENS` reader - Transmitter Enable Status"]
pub type TxensR = crate::BitReader;
#[doc = "Field `RXBLOCK` reader - Block Incoming Data"]
pub type RxblockR = crate::BitReader;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub type TxblR = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub type RxdatavR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline(always)]
    pub fn rxens(&self) -> RxensR {
        RxensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline(always)]
    pub fn txens(&self) -> TxensR {
        TxensR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RxblockR {
        RxblockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x10"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x10;
}
