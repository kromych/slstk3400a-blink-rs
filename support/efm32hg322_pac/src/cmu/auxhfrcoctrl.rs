#[doc = "Register `AUXHFRCOCTRL` reader"]
pub type R = crate::R<AUXHFRCOCTRL_SPEC>;
#[doc = "Register `AUXHFRCOCTRL` writer"]
pub type W = crate::W<AUXHFRCOCTRL_SPEC>;
#[doc = "Field `TUNING` reader - AUXHFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader;
#[doc = "Field `TUNING` writer - AUXHFRCO Tuning Value"]
pub type TUNING_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BAND` reader - AUXHFRCO Band Select"]
pub type BAND_R = crate::FieldReader<BAND_A>;
#[doc = "AUXHFRCO Band Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BAND_A {
    #[doc = "0: 14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14MHZ = 0,
    #[doc = "1: 11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11MHZ = 1,
    #[doc = "2: 7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7MHZ = 2,
    #[doc = "3: 1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1MHZ = 3,
    #[doc = "7: 21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21MHZ = 7,
}
impl From<BAND_A> for u8 {
    #[inline(always)]
    fn from(variant: BAND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BAND_A {
    type Ux = u8;
}
impl BAND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAND_A> {
        match self.bits {
            0 => Some(BAND_A::_14MHZ),
            1 => Some(BAND_A::_11MHZ),
            2 => Some(BAND_A::_7MHZ),
            3 => Some(BAND_A::_1MHZ),
            7 => Some(BAND_A::_21MHZ),
            _ => None,
        }
    }
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_14mhz(&self) -> bool {
        *self == BAND_A::_14MHZ
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_11mhz(&self) -> bool {
        *self == BAND_A::_11MHZ
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_7mhz(&self) -> bool {
        *self == BAND_A::_7MHZ
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == BAND_A::_1MHZ
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_21mhz(&self) -> bool {
        *self == BAND_A::_21MHZ
    }
}
#[doc = "Field `BAND` writer - AUXHFRCO Band Select"]
pub type BAND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, BAND_A>;
impl<'a, REG, const O: u8> BAND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _14mhz(self) -> &'a mut crate::W<REG> {
        self.variant(BAND_A::_14MHZ)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _11mhz(self) -> &'a mut crate::W<REG> {
        self.variant(BAND_A::_11MHZ)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _7mhz(self) -> &'a mut crate::W<REG> {
        self.variant(BAND_A::_7MHZ)
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(BAND_A::_1MHZ)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _21mhz(self) -> &'a mut crate::W<REG> {
        self.variant(BAND_A::_21MHZ)
    }
}
impl R {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<AUXHFRCOCTRL_SPEC, 0> {
        TUNING_W::new(self)
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    #[must_use]
    pub fn band(&mut self) -> BAND_W<AUXHFRCOCTRL_SPEC, 8> {
        BAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AUXHFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxhfrcoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxhfrcoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUXHFRCOCTRL_SPEC;
impl crate::RegisterSpec for AUXHFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxhfrcoctrl::R`](R) reader structure"]
impl crate::Readable for AUXHFRCOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`auxhfrcoctrl::W`](W) writer structure"]
impl crate::Writable for AUXHFRCOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXHFRCOCTRL to value 0x80"]
impl crate::Resettable for AUXHFRCOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
