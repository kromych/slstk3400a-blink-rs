#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `TXC` reader - TX Complete Interrupt Enable"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `TXC` writer - TX Complete Interrupt Enable"]
pub type TXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Enable"]
pub type TXBL_R = crate::BitReader;
#[doc = "Field `TXBL` writer - TX Buffer Level Interrupt Enable"]
pub type TXBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Enable"]
pub type RXDATAV_R = crate::BitReader;
#[doc = "Field `RXDATAV` writer - RX Data Valid Interrupt Enable"]
pub type RXDATAV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX Buffer Full Interrupt Enable"]
pub type RXFULL_R = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX Buffer Full Interrupt Enable"]
pub type RXFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Enable"]
pub type RXOF_R = crate::BitReader;
#[doc = "Field `RXOF` writer - RX Overflow Interrupt Enable"]
pub type RXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Enable"]
pub type RXUF_R = crate::BitReader;
#[doc = "Field `RXUF` writer - RX Underflow Interrupt Enable"]
pub type RXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Enable"]
pub type TXOF_R = crate::BitReader;
#[doc = "Field `TXOF` writer - TX Overflow Interrupt Enable"]
pub type TXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` reader - TX Underflow Interrupt Enable"]
pub type TXUF_R = crate::BitReader;
#[doc = "Field `TXUF` writer - TX Underflow Interrupt Enable"]
pub type TXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Parity Error Interrupt Enable"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error Interrupt Enable"]
pub type PERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Framing Error Interrupt Enable"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `FERR` writer - Framing Error Interrupt Enable"]
pub type FERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt Enable"]
pub type MPAF_R = crate::BitReader;
#[doc = "Field `MPAF` writer - Multi-Processor Address Frame Interrupt Enable"]
pub type MPAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - Slave-Select In Master Mode Interrupt Enable"]
pub type SSM_R = crate::BitReader;
#[doc = "Field `SSM` writer - Slave-Select In Master Mode Interrupt Enable"]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` reader - Collision Check Fail Interrupt Enable"]
pub type CCF_R = crate::BitReader;
#[doc = "Field `CCF` writer - Collision Check Fail Interrupt Enable"]
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txuf(&self) -> TXUF_R {
        TXUF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave-Select In Master Mode Interrupt Enable"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Enable"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IEN_SPEC> {
        TXC_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbl(&mut self) -> TXBL_W<IEN_SPEC> {
        TXBL_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdatav(&mut self) -> RXDATAV_W<IEN_SPEC> {
        RXDATAV_W::new(self, 2)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IEN_SPEC> {
        RXFULL_W::new(self, 3)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<IEN_SPEC> {
        RXOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IEN_SPEC> {
        RXUF_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IEN_SPEC> {
        TXOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TXUF_W<IEN_SPEC> {
        TXUF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<IEN_SPEC> {
        PERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IEN_SPEC> {
        FERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<IEN_SPEC> {
        MPAF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Slave-Select In Master Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<IEN_SPEC> {
        SSM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<IEN_SPEC> {
        CCF_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
