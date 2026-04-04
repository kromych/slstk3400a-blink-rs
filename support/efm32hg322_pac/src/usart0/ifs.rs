#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `TXC` writer - Set TX Complete Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Set RX Buffer Full Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Set RX Overflow Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RX Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TX Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` writer - Set TX Underflow Interrupt Flag"]
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Set Parity Error Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Set Framing Error Interrupt Flag"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Set Multi-Processor Address Frame Interrupt Flag"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` writer - Set Slave-Select in Master mode Interrupt Flag"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` writer - Set Collision Check Fail Interrupt Flag"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfsSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 3 - Set RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IfsSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - Set RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IfsSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - Set RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfsSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - Set TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfsSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 7 - Set TX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TxufW<'_, IfsSpec> {
        TxufW::new(self, 7)
    }
    #[doc = "Bit 8 - Set Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IfsSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Set Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IfsSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Set Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IfsSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 11 - Set Slave-Select in Master mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<'_, IfsSpec> {
        SsmW::new(self, 11)
    }
    #[doc = "Bit 12 - Set Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<'_, IfsSpec> {
        CcfW::new(self, 12)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
