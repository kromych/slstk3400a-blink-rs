#[doc = "Register `DOEP0CTL` reader"]
pub type R = crate::R<DOEP0CTL_SPEC>;
#[doc = "Register `DOEP0CTL` writer"]
pub type W = crate::W<DOEP0CTL_SPEC>;
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
    pub fn variant(&self) -> MPS_A {
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
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type USBACTEP_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `SNP` reader - Snoop Mode"]
pub type SNP_R = crate::BitReader;
#[doc = "Field `SNP` writer - Snoop Mode"]
pub type SNP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALL` reader - Handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - Handshake"]
pub type STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EPENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
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
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<DOEP0CTL_SPEC, 20> {
        SNP_W::new(self)
    }
    #[doc = "Bit 21 - Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEP0CTL_SPEC, 21> {
        STALL_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEP0CTL_SPEC, 26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEP0CTL_SPEC, 27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DOEP0CTL_SPEC, 31> {
        EPENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device OUT Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP0CTL_SPEC;
impl crate::RegisterSpec for DOEP0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep0ctl::R`](R) reader structure"]
impl crate::Readable for DOEP0CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep0ctl::W`](W) writer structure"]
impl crate::Writable for DOEP0CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEP0CTL to value 0x8000"]
impl crate::Resettable for DOEP0CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
