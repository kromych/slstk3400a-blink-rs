#[doc = "Register `DIEP0CTL` reader"]
pub type R = crate::R<DIEP0CTL_SPEC>;
#[doc = "Register `DIEP0CTL` writer"]
pub type W = crate::W<DIEP0CTL_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<MPS_A>;
#[doc = "Maximum Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MPS_A {
    #[doc = "0: 64 bytes."]
    _64B = 0,
    #[doc = "1: 32 bytes."]
    _32B = 1,
    #[doc = "2: 16 bytes."]
    _16B = 2,
    #[doc = "3: 8 bytes."]
    _8B = 3,
}
impl From<MPS_A> for u8 {
    #[inline(always)]
    fn from(variant: MPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MPS_A {
    type Ux = u8;
}
impl MPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MPS_A {
        match self.bits {
            0 => MPS_A::_64B,
            1 => MPS_A::_32B,
            2 => MPS_A::_16B,
            3 => MPS_A::_8B,
            _ => unreachable!(),
        }
    }
    #[doc = "64 bytes."]
    #[inline(always)]
    pub fn is_64b(&self) -> bool {
        *self == MPS_A::_64B
    }
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn is_32b(&self) -> bool {
        *self == MPS_A::_32B
    }
    #[doc = "16 bytes."]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == MPS_A::_16B
    }
    #[doc = "8 bytes."]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == MPS_A::_8B
    }
}
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MPS_A>;
impl<'a, REG> MPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64 bytes."]
    #[inline(always)]
    pub fn _64b(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::_64B)
    }
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn _32b(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::_32B)
    }
    #[doc = "16 bytes."]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::_16B)
    }
    #[doc = "8 bytes."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::_8B)
    }
}
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type USBACTEP_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `STALL` reader - Handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO Number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO Number"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DIEP0CTL_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DIEP0CTL_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<DIEP0CTL_SPEC> {
        TXFNUM_W::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DIEP0CTL_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DIEP0CTL_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<DIEP0CTL_SPEC> {
        EPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DIEP0CTL_SPEC> {
        EPENA_W::new(self, 31)
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
#[doc = "Device IN Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP0CTL_SPEC;
impl crate::RegisterSpec for DIEP0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0ctl::R`](R) reader structure"]
impl crate::Readable for DIEP0CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep0ctl::W`](W) writer structure"]
impl crate::Writable for DIEP0CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEP0CTL to value 0x8000"]
impl crate::Resettable for DIEP0CTL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
