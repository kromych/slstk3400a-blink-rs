#[doc = "Register `DIEP0_TXFSTS` reader"]
pub type R = crate::R<Diep0TxfstsSpec>;
#[doc = "Field `SPCAVAIL` reader - TxFIFO Space Available"]
pub type SpcavailR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TxFIFO Space Available"]
    #[inline(always)]
    pub fn spcavail(&self) -> SpcavailR {
        SpcavailR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_txfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0TxfstsSpec;
impl crate::RegisterSpec for Diep0TxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0_txfsts::R`](R) reader structure"]
impl crate::Readable for Diep0TxfstsSpec {}
#[doc = "`reset()` method sets DIEP0_TXFSTS to value 0x0200"]
impl crate::Resettable for Diep0TxfstsSpec {
    const RESET_VALUE: u32 = 0x0200;
}
