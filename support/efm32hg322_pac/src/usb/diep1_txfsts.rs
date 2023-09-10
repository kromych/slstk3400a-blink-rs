#[doc = "Register `DIEP1_TXFSTS` reader"]
pub type R = crate::R<DIEP1_TXFSTS_SPEC>;
#[doc = "Field `SPCAVAIL` reader - TxFIFO Space Available"]
pub type SPCAVAIL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TxFIFO Space Available"]
    #[inline(always)]
    pub fn spcavail(&self) -> SPCAVAIL_R {
        SPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_txfsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP1_TXFSTS_SPEC;
impl crate::RegisterSpec for DIEP1_TXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep1_txfsts::R`](R) reader structure"]
impl crate::Readable for DIEP1_TXFSTS_SPEC {}
#[doc = "`reset()` method sets DIEP1_TXFSTS to value 0x0200"]
impl crate::Resettable for DIEP1_TXFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
