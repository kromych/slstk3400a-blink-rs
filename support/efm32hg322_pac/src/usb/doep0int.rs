#[doc = "Register `DOEP0INT` reader"]
pub type R = crate::R<Doep0intSpec>;
#[doc = "Register `DOEP0INT` writer"]
pub type W = crate::W<Doep0intSpec>;
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
#[doc = "Field `SETUP` reader - Setup Phase Done"]
pub type SetupR = crate::BitReader;
#[doc = "Field `SETUP` writer - Setup Phase Done"]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDIS` reader - OUT Token Received When Endpoint Disabled"]
pub type OuttknepdisR = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS` writer - OUT Token Received When Endpoint Disabled"]
pub type OuttknepdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVD` reader - Status Phase Received For Control Write"]
pub type StsphsercvdR = crate::BitReader;
#[doc = "Field `STSPHSERCVD` writer - Status Phase Received For Control Write"]
pub type StsphsercvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP` reader - Back-to-Back SETUP Packets Received"]
pub type Back2backsetupR = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP` writer - Back-to-Back SETUP Packets Received"]
pub type Back2backsetupW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `STUPPKTRCVD` reader - Setup Packet Received"]
pub type StuppktrcvdR = crate::BitReader;
#[doc = "Field `STUPPKTRCVD` writer - Setup Packet Received"]
pub type StuppktrcvdW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Setup Phase Done"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtknepdis(&self) -> OuttknepdisR {
        OuttknepdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn stsphsercvd(&self) -> StsphsercvdR {
        StsphsercvdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> Back2backsetupR {
        Back2backsetupR::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 15 - Setup Packet Received"]
    #[inline(always)]
    pub fn stuppktrcvd(&self) -> StuppktrcvdR {
        StuppktrcvdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XfercomplW<'_, Doep0intSpec> {
        XfercomplW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EpdisbldW<'_, Doep0intSpec> {
        EpdisbldW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AhberrW<'_, Doep0intSpec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - Setup Phase Done"]
    #[inline(always)]
    pub fn setup(&mut self) -> SetupW<'_, Doep0intSpec> {
        SetupW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtknepdis(&mut self) -> OuttknepdisW<'_, Doep0intSpec> {
        OuttknepdisW::new(self, 4)
    }
    #[doc = "Bit 5 - Status Phase Received For Control Write"]
    #[inline(always)]
    pub fn stsphsercvd(&mut self) -> StsphsercvdW<'_, Doep0intSpec> {
        StsphsercvdW::new(self, 5)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2backsetup(&mut self) -> Back2backsetupW<'_, Doep0intSpec> {
        Back2backsetupW::new(self, 6)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PktdrpstsW<'_, Doep0intSpec> {
        PktdrpstsW::new(self, 11)
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&mut self) -> BbleerrW<'_, Doep0intSpec> {
        BbleerrW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK Interrupt"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NakintrptW<'_, Doep0intSpec> {
        NakintrptW::new(self, 13)
    }
    #[doc = "Bit 15 - Setup Packet Received"]
    #[inline(always)]
    pub fn stuppktrcvd(&mut self) -> StuppktrcvdW<'_, Doep0intSpec> {
        StuppktrcvdW::new(self, 15)
    }
}
#[doc = "Device OUT Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep0intSpec;
impl crate::RegisterSpec for Doep0intSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep0int::R`](R) reader structure"]
impl crate::Readable for Doep0intSpec {}
#[doc = "`write(|w| ..)` method takes [`doep0int::W`](W) writer structure"]
impl crate::Writable for Doep0intSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP0INT to value 0"]
impl crate::Resettable for Doep0intSpec {}
