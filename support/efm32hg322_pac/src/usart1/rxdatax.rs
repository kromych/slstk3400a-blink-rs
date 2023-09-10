#[doc = "Register `RXDATAX` reader"]
pub type R = crate::R<RXDATAX_SPEC>;
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RXDATA_R = crate::FieldReader<u16>;
#[doc = "Field `PERR` reader - Data Parity Error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Data Framing Error"]
pub type FERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "RX Buffer Data Extended Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatax::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATAX_SPEC;
impl crate::RegisterSpec for RXDATAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdatax::R`](R) reader structure"]
impl crate::Readable for RXDATAX_SPEC {}
#[doc = "`reset()` method sets RXDATAX to value 0"]
impl crate::Resettable for RXDATAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
