#[doc = "Register `DOEP0CTL` reader"]
pub type R = crate::R<Doep0ctlSpec>;
#[doc = "Register `DOEP0CTL` writer"]
pub type W = crate::W<Doep0ctlSpec>;
#[doc = "Maximum Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mps {
    #[doc = "0: 64 bytes."]
    _64b = 0,
    #[doc = "1: 32 bytes."]
    _32b = 1,
    #[doc = "2: 16 bytes."]
    _16b = 2,
    #[doc = "3: 8 bytes."]
    _8b = 3,
}
impl From<Mps> for u8 {
    #[inline(always)]
    fn from(variant: Mps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mps {
    type Ux = u8;
}
impl crate::IsEnum for Mps {}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<Mps>;
impl MpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mps {
        match self.bits {
            0 => Mps::_64b,
            1 => Mps::_32b,
            2 => Mps::_16b,
            3 => Mps::_8b,
            _ => unreachable!(),
        }
    }
    #[doc = "64 bytes."]
    #[inline(always)]
    pub fn is_64b(&self) -> bool {
        *self == Mps::_64b
    }
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == Mps::_32b
    }
    #[doc = "16 bytes."]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == Mps::_16b
    }
    #[doc = "8 bytes."]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == Mps::_8b
    }
}
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type UsbactepR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `SNP` reader - Snoop Mode"]
pub type SnpR = crate::BitReader;
#[doc = "Field `SNP` writer - Snoop Mode"]
pub type SnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> UsbactepR {
        UsbactepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&self) -> SnpR {
        SnpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&mut self) -> SnpW<'_, Doep0ctlSpec> {
        SnpW::new(self, 20)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Doep0ctlSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Doep0ctlSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Doep0ctlSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Doep0ctlSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "Device OUT Endpoint 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep0ctlSpec;
impl crate::RegisterSpec for Doep0ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep0ctl::R`](R) reader structure"]
impl crate::Readable for Doep0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`doep0ctl::W`](W) writer structure"]
impl crate::Writable for Doep0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP0CTL to value 0x8000"]
impl crate::Resettable for Doep0ctlSpec {
    const RESET_VALUE: u32 = 0x8000;
}
