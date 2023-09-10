#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `RXENS` reader - Receiver Enable Status"]
pub type RXENS_R = crate::BitReader;
#[doc = "Field `TXENS` reader - Transmitter Enable Status"]
pub type TXENS_R = crate::BitReader;
#[doc = "Field `MASTER` reader - SPI Master Mode"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `RXBLOCK` reader - Block Incoming Data"]
pub type RXBLOCK_R = crate::BitReader;
#[doc = "Field `TXTRI` reader - Transmitter Tristated"]
pub type TXTRI_R = crate::BitReader;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub type TXBL_R = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub type RXDATAV_R = crate::BitReader;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RXFULL_R = crate::BitReader;
#[doc = "Field `TXBDRIGHT` reader - TX Buffer Expects Double Right Data"]
pub type TXBDRIGHT_R = crate::BitReader;
#[doc = "Field `TXBSRIGHT` reader - TX Buffer Expects Single Right Data"]
pub type TXBSRIGHT_R = crate::BitReader;
#[doc = "Field `RXDATAVRIGHT` reader - RX Data Right"]
pub type RXDATAVRIGHT_R = crate::BitReader;
#[doc = "Field `RXFULLRIGHT` reader - RX Full of Right Data"]
pub type RXFULLRIGHT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline(always)]
    pub fn rxens(&self) -> RXENS_R {
        RXENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline(always)]
    pub fn txens(&self) -> TXENS_R {
        TXENS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RXBLOCK_R {
        RXBLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter Tristated"]
    #[inline(always)]
    pub fn txtri(&self) -> TXTRI_R {
        TXTRI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX Buffer Expects Double Right Data"]
    #[inline(always)]
    pub fn txbdright(&self) -> TXBDRIGHT_R {
        TXBDRIGHT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX Buffer Expects Single Right Data"]
    #[inline(always)]
    pub fn txbsright(&self) -> TXBSRIGHT_R {
        TXBSRIGHT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX Data Right"]
    #[inline(always)]
    pub fn rxdatavright(&self) -> RXDATAVRIGHT_R {
        RXDATAVRIGHT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX Full of Right Data"]
    #[inline(always)]
    pub fn rxfullright(&self) -> RXFULLRIGHT_R {
        RXFULLRIGHT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "USART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
