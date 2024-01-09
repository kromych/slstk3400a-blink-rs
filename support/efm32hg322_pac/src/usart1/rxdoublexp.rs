#[doc = "Register `RXDOUBLEXP` reader"]
pub type R = crate::R<RXDOUBLEXP_SPEC>;
#[doc = "Field `RXDATAP0` reader - RX Data 0 Peek"]
pub type RXDATAP0_R = crate::FieldReader<u16>;
#[doc = "Field `PERRP0` reader - Data Parity Error 0 Peek"]
pub type PERRP0_R = crate::BitReader;
#[doc = "Field `FERRP0` reader - Data Framing Error 0 Peek"]
pub type FERRP0_R = crate::BitReader;
#[doc = "Field `RXDATAP1` reader - RX Data 1 Peek"]
pub type RXDATAP1_R = crate::FieldReader<u16>;
#[doc = "Field `PERRP1` reader - Data Parity Error 1 Peek"]
pub type PERRP1_R = crate::BitReader;
#[doc = "Field `FERRP1` reader - Data Framing Error 1 Peek"]
pub type FERRP1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data 0 Peek"]
    #[inline(always)]
    pub fn rxdatap0(&self) -> RXDATAP0_R {
        RXDATAP0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error 0 Peek"]
    #[inline(always)]
    pub fn perrp0(&self) -> PERRP0_R {
        PERRP0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error 0 Peek"]
    #[inline(always)]
    pub fn ferrp0(&self) -> FERRP0_R {
        FERRP0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - RX Data 1 Peek"]
    #[inline(always)]
    pub fn rxdatap1(&self) -> RXDATAP1_R {
        RXDATAP1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Data Parity Error 1 Peek"]
    #[inline(always)]
    pub fn perrp1(&self) -> PERRP1_R {
        PERRP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Framing Error 1 Peek"]
    #[inline(always)]
    pub fn ferrp1(&self) -> FERRP1_R {
        FERRP1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "RX Buffer Double Data Extended Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdoublexp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDOUBLEXP_SPEC;
impl crate::RegisterSpec for RXDOUBLEXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdoublexp::R`](R) reader structure"]
impl crate::Readable for RXDOUBLEXP_SPEC {}
#[doc = "`reset()` method sets RXDOUBLEXP to value 0"]
impl crate::Resettable for RXDOUBLEXP_SPEC {
    const RESET_VALUE: u32 = 0;
}
