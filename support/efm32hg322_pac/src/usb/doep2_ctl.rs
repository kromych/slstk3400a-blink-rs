#[doc = "Register `DOEP2_CTL` reader"]
pub type R = crate::R<DOEP2_CTL_SPEC>;
#[doc = "Register `DOEP2_CTL` writer"]
pub type W = crate::W<DOEP2_CTL_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint"]
pub type USBACTEP_R = crate::BitReader;
#[doc = "Field `USBACTEP` writer - USB Active Endpoint"]
pub type USBACTEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPIDEOF` reader - Endpoint Data PID / Even-odd Frame"]
pub type DPIDEOF_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<EPTYPE_A>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control Endpoint."]
    CONTROL = 0,
    #[doc = "1: Isochronous Endpoint."]
    ISO = 1,
    #[doc = "2: Bulk Endpoint."]
    BULK = 2,
    #[doc = "3: Interrupt Endpoint."]
    INT = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPTYPE_A {
    type Ux = u8;
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::CONTROL,
            1 => EPTYPE_A::ISO,
            2 => EPTYPE_A::BULK,
            3 => EPTYPE_A::INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Control Endpoint."]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EPTYPE_A::CONTROL
    }
    #[doc = "Isochronous Endpoint."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EPTYPE_A::ISO
    }
    #[doc = "Bulk Endpoint."]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EPTYPE_A::BULK
    }
    #[doc = "Interrupt Endpoint."]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == EPTYPE_A::INT
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EPTYPE_A>;
impl<'a, REG> EPTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control Endpoint."]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::CONTROL)
    }
    #[doc = "Isochronous Endpoint."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::ISO)
    }
    #[doc = "Bulk Endpoint."]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::BULK)
    }
    #[doc = "Interrupt Endpoint."]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::INT)
    }
}
#[doc = "Field `SNP` reader - Snoop Mode"]
pub type SNP_R = crate::BitReader;
#[doc = "Field `SNP` writer - Snoop Mode"]
pub type SNP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL Handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PIDEF` writer - Set DATA0 PID / Even Frame"]
pub type SETD0PIDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PIDOF` writer - Set DATA1 PID / Odd Frame"]
pub type SETD1PIDOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable"]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable"]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID / Even-odd Frame"]
    #[inline(always)]
    pub fn dpideof(&self) -> DPIDEOF_R {
        DPIDEOF_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 21 - STALL Handshake"]
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DOEP2_CTL_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbactep(&mut self) -> USBACTEP_W<DOEP2_CTL_SPEC> {
        USBACTEP_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DOEP2_CTL_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<DOEP2_CTL_SPEC> {
        SNP_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEP2_CTL_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEP2_CTL_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEP2_CTL_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID / Even Frame"]
    #[inline(always)]
    #[must_use]
    pub fn setd0pidef(&mut self) -> SETD0PIDEF_W<DOEP2_CTL_SPEC> {
        SETD0PIDEF_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set DATA1 PID / Odd Frame"]
    #[inline(always)]
    #[must_use]
    pub fn setd1pidof(&mut self) -> SETD1PIDOF_W<DOEP2_CTL_SPEC> {
        SETD1PIDOF_W::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<DOEP2_CTL_SPEC> {
        EPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DOEP2_CTL_SPEC> {
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
#[doc = "Device OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP2_CTL_SPEC;
impl crate::RegisterSpec for DOEP2_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep2_ctl::R`](R) reader structure"]
impl crate::Readable for DOEP2_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep2_ctl::W`](W) writer structure"]
impl crate::Writable for DOEP2_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEP2_CTL to value 0"]
impl crate::Resettable for DOEP2_CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
