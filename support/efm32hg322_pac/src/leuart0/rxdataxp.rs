#[doc = "Register `RXDATAXP` reader"]
pub type R = crate::R<RXDATAXP_SPEC>;
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RXDATAP_R = crate::FieldReader<u16>;
#[doc = "Field `PERRP` reader - Receive Data Parity Error Peek"]
pub type PERRP_R = crate::BitReader;
#[doc = "Field `FERRP` reader - Receive Data Framing Error Peek"]
pub type FERRP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Receive Data Parity Error Peek"]
    #[inline(always)]
    pub fn perrp(&self) -> PERRP_R {
        PERRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive Data Framing Error Peek"]
    #[inline(always)]
    pub fn ferrp(&self) -> FERRP_R {
        FERRP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Receive Buffer Data Extended Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdataxp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATAXP_SPEC;
impl crate::RegisterSpec for RXDATAXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdataxp::R`](R) reader structure"]
impl crate::Readable for RXDATAXP_SPEC {}
#[doc = "`reset()` method sets RXDATAXP to value 0"]
impl crate::Resettable for RXDATAXP_SPEC {
    const RESET_VALUE: u32 = 0;
}
