#[doc = "Register `RXDATAP` reader"]
pub type R = crate::R<RXDATAP_SPEC>;
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RXDATAP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Buffer Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATAP_SPEC;
impl crate::RegisterSpec for RXDATAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdatap::R`](R) reader structure"]
impl crate::Readable for RXDATAP_SPEC {}
#[doc = "`reset()` method sets RXDATAP to value 0"]
impl crate::Resettable for RXDATAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
