#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `TXC` writer - Set TX Complete Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Set RX Overflow Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RX Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TX Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Set Parity Error Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Set Framing Error Interrupt Flag"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Set Multi-Processor Address Frame Interrupt Flag"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTF` writer - Set Start Frame Interrupt Flag"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGF` writer - Set Signal Frame Interrupt Flag"]
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfsSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 3 - Set RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IfsSpec> {
        RxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Set RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfsSpec> {
        RxufW::new(self, 4)
    }
    #[doc = "Bit 5 - Set TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfsSpec> {
        TxofW::new(self, 5)
    }
    #[doc = "Bit 6 - Set Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IfsSpec> {
        PerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Set Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IfsSpec> {
        FerrW::new(self, 7)
    }
    #[doc = "Bit 8 - Set Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IfsSpec> {
        MpafW::new(self, 8)
    }
    #[doc = "Bit 9 - Set Start Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<'_, IfsSpec> {
        StartfW::new(self, 9)
    }
    #[doc = "Bit 10 - Set Signal Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SigfW<'_, IfsSpec> {
        SigfW::new(self, 10)
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
