#[doc = "Register `TRACECTRL` reader"]
pub type R = crate::R<TRACECTRL_SPEC>;
#[doc = "Register `TRACECTRL` writer"]
pub type W = crate::W<TRACECTRL_SPEC>;
#[doc = "Field `TSTARTEN` reader - PRS TSTART Enable"]
pub type TSTARTEN_R = crate::BitReader;
#[doc = "Field `TSTARTEN` writer - PRS TSTART Enable"]
pub type TSTARTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTART` reader - MTB TSTART PRS select"]
pub type TSTART_R = crate::FieldReader<TSTART_A>;
#[doc = "MTB TSTART PRS select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTART_A {
    #[doc = "0: PRS ch 0 is controlling TSTART."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 is controlling TSTART."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 is controlling TSTART."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 is controlling TSTART."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 is controlling TSTART."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 is controlling TSTART."]
    PRSCH5 = 5,
}
impl From<TSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTART_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSTART_A {
    type Ux = u8;
}
impl TSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSTART_A> {
        match self.bits {
            0 => Some(TSTART_A::PRSCH0),
            1 => Some(TSTART_A::PRSCH1),
            2 => Some(TSTART_A::PRSCH2),
            3 => Some(TSTART_A::PRSCH3),
            4 => Some(TSTART_A::PRSCH4),
            5 => Some(TSTART_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTART_A::PRSCH0
    }
    #[doc = "PRS ch 1 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTART_A::PRSCH1
    }
    #[doc = "PRS ch 2 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTART_A::PRSCH2
    }
    #[doc = "PRS ch 3 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTART_A::PRSCH3
    }
    #[doc = "PRS ch 4 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTART_A::PRSCH4
    }
    #[doc = "PRS ch 5 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTART_A::PRSCH5
    }
}
#[doc = "Field `TSTART` writer - MTB TSTART PRS select"]
pub type TSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTART_A>;
impl<'a, REG> TSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART_A::PRSCH5)
    }
}
#[doc = "Field `TSTOPEN` reader - PRS TSTOP Enable"]
pub type TSTOPEN_R = crate::BitReader;
#[doc = "Field `TSTOPEN` writer - PRS TSTOP Enable"]
pub type TSTOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTOP` reader - MTB TSTOP PRS select"]
pub type TSTOP_R = crate::FieldReader<TSTOP_A>;
#[doc = "MTB TSTOP PRS select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP_A {
    #[doc = "0: PRS ch 0 is controlling TSTOP."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 is controlling TSTOP."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 is controlling TSTOP."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 is controlling TSTOP."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 is controlling TSTOP."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 is controlling TSTOP."]
    PRSCH5 = 5,
}
impl From<TSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSTOP_A {
    type Ux = u8;
}
impl TSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSTOP_A> {
        match self.bits {
            0 => Some(TSTOP_A::PRSCH0),
            1 => Some(TSTOP_A::PRSCH1),
            2 => Some(TSTOP_A::PRSCH2),
            3 => Some(TSTOP_A::PRSCH3),
            4 => Some(TSTOP_A::PRSCH4),
            5 => Some(TSTOP_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TSTOP_A::PRSCH0
    }
    #[doc = "PRS ch 1 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TSTOP_A::PRSCH1
    }
    #[doc = "PRS ch 2 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TSTOP_A::PRSCH2
    }
    #[doc = "PRS ch 3 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TSTOP_A::PRSCH3
    }
    #[doc = "PRS ch 4 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TSTOP_A::PRSCH4
    }
    #[doc = "PRS ch 5 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TSTOP_A::PRSCH5
    }
}
#[doc = "Field `TSTOP` writer - MTB TSTOP PRS select"]
pub type TSTOP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTOP_A>;
impl<'a, REG> TSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::PRSCH0)
    }
    #[doc = "PRS ch 1 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::PRSCH1)
    }
    #[doc = "PRS ch 2 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::PRSCH2)
    }
    #[doc = "PRS ch 3 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::PRSCH3)
    }
    #[doc = "PRS ch 4 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::PRSCH4)
    }
    #[doc = "PRS ch 5 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP_A::PRSCH5)
    }
}
impl R {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstarten(&mut self) -> TSTARTEN_W<TRACECTRL_SPEC> {
        TSTARTEN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<TRACECTRL_SPEC> {
        TSTART_W::new(self, 1)
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstopen(&mut self) -> TSTOPEN_W<TRACECTRL_SPEC> {
        TSTOPEN_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<TRACECTRL_SPEC> {
        TSTOP_W::new(self, 9)
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
#[doc = "MTB Trace Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACECTRL_SPEC;
impl crate::RegisterSpec for TRACECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tracectrl::R`](R) reader structure"]
impl crate::Readable for TRACECTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tracectrl::W`](W) writer structure"]
impl crate::Writable for TRACECTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACECTRL to value 0"]
impl crate::Resettable for TRACECTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
