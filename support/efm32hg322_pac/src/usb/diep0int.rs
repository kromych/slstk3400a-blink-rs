#[doc = "Register `DIEP0INT` reader"]
pub type R = crate::R<Diep0intSpec>;
#[doc = "Register `DIEP0INT` writer"]
pub type W = crate::W<Diep0intSpec>;
#[doc = "Field `XFERCOMPL` reader - Transfer Completed Interrupt"]
pub type XfercomplR = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed Interrupt"]
pub type XfercomplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLD` reader - Endpoint Disabled Interrupt"]
pub type EpdisbldR = crate::BitReader;
#[doc = "Field `EPDISBLD` writer - Endpoint Disabled Interrupt"]
pub type EpdisbldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AhberrR = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Timeout Condition"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout Condition"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMP` reader - IN Token Received When TxFIFO is Empty"]
pub type IntkntxfempR = crate::BitReader;
#[doc = "Field `INTKNTXFEMP` writer - IN Token Received When TxFIFO is Empty"]
pub type IntkntxfempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNAKEFF` reader - IN Endpoint NAK Effective"]
pub type InepnakeffR = crate::BitReader;
#[doc = "Field `INEPNAKEFF` writer - IN Endpoint NAK Effective"]
pub type InepnakeffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMP` reader - Transmit FIFO Empty"]
pub type TxfempR = crate::BitReader;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PktdrpstsR = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PktdrpstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR` reader - NAK Interrupt"]
pub type BbleerrR = crate::BitReader;
#[doc = "Field `BBLEERR` writer - NAK Interrupt"]
pub type BbleerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT` reader - NAK Interrupt"]
pub type NakintrptR = crate::BitReader;
#[doc = "Field `NAKINTRPT` writer - NAK Interrupt"]
pub type NakintrptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XfercomplR {
        XfercomplR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EpdisbldR {
        EpdisbldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AhberrR {
        AhberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkntxfemp(&self) -> IntkntxfempR {
        IntkntxfempR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnakeff(&self) -> InepnakeffR {
        InepnakeffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txfemp(&self) -> TxfempR {
        TxfempR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PktdrpstsR {
        PktdrpstsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BbleerrR {
        BbleerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NakintrptR {
        NakintrptR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XfercomplW<'_, Diep0intSpec> {
        XfercomplW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EpdisbldW<'_, Diep0intSpec> {
        EpdisbldW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AhberrW<'_, Diep0intSpec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, Diep0intSpec> {
        TimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO is Empty"]
    #[inline(always)]
    pub fn intkntxfemp(&mut self) -> IntkntxfempW<'_, Diep0intSpec> {
        IntkntxfempW::new(self, 4)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective"]
    #[inline(always)]
    pub fn inepnakeff(&mut self) -> InepnakeffW<'_, Diep0intSpec> {
        InepnakeffW::new(self, 6)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PktdrpstsW<'_, Diep0intSpec> {
        PktdrpstsW::new(self, 11)
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&mut self) -> BbleerrW<'_, Diep0intSpec> {
        BbleerrW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NakintrptW<'_, Diep0intSpec> {
        NakintrptW::new(self, 13)
    }
}
#[doc = "Device IN Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0intSpec;
impl crate::RegisterSpec for Diep0intSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0int::R`](R) reader structure"]
impl crate::Readable for Diep0intSpec {}
#[doc = "`write(|w| ..)` method takes [`diep0int::W`](W) writer structure"]
impl crate::Writable for Diep0intSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP0INT to value 0x80"]
impl crate::Resettable for Diep0intSpec {
    const RESET_VALUE: u32 = 0x80;
}
