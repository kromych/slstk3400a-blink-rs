#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `PSTART` reader - Pending START"]
pub type PSTART_R = crate::BitReader;
#[doc = "Field `PSTOP` reader - Pending STOP"]
pub type PSTOP_R = crate::BitReader;
#[doc = "Field `PACK` reader - Pending ACK"]
pub type PACK_R = crate::BitReader;
#[doc = "Field `PNACK` reader - Pending NACK"]
pub type PNACK_R = crate::BitReader;
#[doc = "Field `PCONT` reader - Pending continue"]
pub type PCONT_R = crate::BitReader;
#[doc = "Field `PABORT` reader - Pending abort"]
pub type PABORT_R = crate::BitReader;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub type TXBL_R = crate::BitReader;
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub type RXDATAV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pending START"]
    #[inline(always)]
    pub fn pstart(&self) -> PSTART_R {
        PSTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending STOP"]
    #[inline(always)]
    pub fn pstop(&self) -> PSTOP_R {
        PSTOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending ACK"]
    #[inline(always)]
    pub fn pack(&self) -> PACK_R {
        PACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending NACK"]
    #[inline(always)]
    pub fn pnack(&self) -> PNACK_R {
        PNACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending continue"]
    #[inline(always)]
    pub fn pcont(&self) -> PCONT_R {
        PCONT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending abort"]
    #[inline(always)]
    pub fn pabort(&self) -> PABORT_R {
        PABORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x80"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
