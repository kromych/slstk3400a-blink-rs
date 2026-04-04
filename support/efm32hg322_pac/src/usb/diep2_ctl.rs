#[doc = "Register `DIEP2_CTL` reader"]
pub type R = crate::R<Diep2CtlSpec>;
#[doc = "Register `DIEP2_CTL` writer"]
pub type W = crate::W<Diep2CtlSpec>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type UsbactepR = crate::BitReader;
#[doc = "Field `USBACTEP` writer - USB Active Endpoint"]
pub type UsbactepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPIDEOF` reader - Endpoint Data PID / Even or Odd Frame"]
pub type DpideofR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NakstsR = crate::BitReader;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptype {
    #[doc = "0: Control Endpoint."]
    Control = 0,
    #[doc = "1: Isochronous Endpoint."]
    Iso = 1,
    #[doc = "2: Bulk Endpoint."]
    Bulk = 2,
    #[doc = "3: Interrupt Endpoint."]
    Int = 3,
}
impl From<Eptype> for u8 {
    #[inline(always)]
    fn from(variant: Eptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptype {
    type Ux = u8;
}
impl crate::IsEnum for Eptype {}
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader<Eptype>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eptype {
        match self.bits {
            0 => Eptype::Control,
            1 => Eptype::Iso,
            2 => Eptype::Bulk,
            3 => Eptype::Int,
            _ => unreachable!(),
        }
    }
    #[doc = "Control Endpoint."]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == Eptype::Control
    }
    #[doc = "Isochronous Endpoint."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Eptype::Iso
    }
    #[doc = "Bulk Endpoint."]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == Eptype::Bulk
    }
    #[doc = "Interrupt Endpoint."]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Eptype::Int
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eptype, crate::Safe>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control Endpoint."]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Control)
    }
    #[doc = "Isochronous Endpoint."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Iso)
    }
    #[doc = "Bulk Endpoint."]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Bulk)
    }
    #[doc = "Interrupt Endpoint."]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Int)
    }
}
#[doc = "Field `STALL` reader - Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO Number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO Number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PIDEF` writer - Set DATA0 PID / Even Frame"]
pub type Setd0pidefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PIDOF` writer - Set DATA1 PID / Odd Frame"]
pub type Setd1pidofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> UsbactepR {
        UsbactepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID / Even or Odd Frame"]
    #[inline(always)]
    pub fn dpideof(&self) -> DpideofR {
        DpideofR::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 22) & 0x0f) as u8)
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MpsW<'_, Diep2CtlSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&mut self) -> UsbactepW<'_, Diep2CtlSpec> {
        UsbactepW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EptypeW<'_, Diep2CtlSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Diep2CtlSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<'_, Diep2CtlSpec> {
        TxfnumW::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<'_, Diep2CtlSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<'_, Diep2CtlSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID / Even Frame"]
    #[inline(always)]
    pub fn setd0pidef(&mut self) -> Setd0pidefW<'_, Diep2CtlSpec> {
        Setd0pidefW::new(self, 28)
    }
    #[doc = "Bit 29 - Set DATA1 PID / Odd Frame"]
    #[inline(always)]
    pub fn setd1pidof(&mut self) -> Setd1pidofW<'_, Diep2CtlSpec> {
        Setd1pidofW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<'_, Diep2CtlSpec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<'_, Diep2CtlSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "Device IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep2CtlSpec;
impl crate::RegisterSpec for Diep2CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep2_ctl::R`](R) reader structure"]
impl crate::Readable for Diep2CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`diep2_ctl::W`](W) writer structure"]
impl crate::Writable for Diep2CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP2_CTL to value 0"]
impl crate::Resettable for Diep2CtlSpec {}
