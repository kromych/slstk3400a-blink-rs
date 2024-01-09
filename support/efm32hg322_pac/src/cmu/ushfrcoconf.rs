#[doc = "Register `USHFRCOCONF` reader"]
pub type R = crate::R<USHFRCOCONF_SPEC>;
#[doc = "Register `USHFRCOCONF` writer"]
pub type W = crate::W<USHFRCOCONF_SPEC>;
#[doc = "Field `BAND` reader - USHFRCO Band Select"]
pub type BAND_R = crate::FieldReader<BAND_A>;
#[doc = "USHFRCO Band Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BAND_A {
    #[doc = "1: 48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _48MHZ = 1,
    #[doc = "3: 24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _24MHZ = 3,
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
    pub const fn variant(&self) -> Option<BAND_A> {
        match self.bits {
            1 => Some(BAND_A::_48MHZ),
            3 => Some(BAND_A::_24MHZ),
            _ => None,
        }
    }
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn is_48mhz(&self) -> bool {
        *self == BAND_A::_48MHZ
    }
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == BAND_A::_24MHZ
    }
}
#[doc = "Field `BAND` writer - USHFRCO Band Select"]
pub type BAND_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BAND_A>;
impl<'a, REG> BAND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(BAND_A::_48MHZ)
    }
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(BAND_A::_24MHZ)
    }
}
#[doc = "Field `USHFRCODIV2DIS` reader - USHFRCO divider for HFCLK disable"]
pub type USHFRCODIV2DIS_R = crate::BitReader;
#[doc = "Field `USHFRCODIV2DIS` writer - USHFRCO divider for HFCLK disable"]
pub type USHFRCODIV2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&self) -> USHFRCODIV2DIS_R {
        USHFRCODIV2DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    #[must_use]
    pub fn band(&mut self) -> BAND_W<USHFRCOCONF_SPEC> {
        BAND_W::new(self, 0)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcodiv2dis(&mut self) -> USHFRCODIV2DIS_W<USHFRCOCONF_SPEC> {
        USHFRCODIV2DIS_W::new(self, 4)
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
#[doc = "USHFRCO Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcoconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ushfrcoconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USHFRCOCONF_SPEC;
impl crate::RegisterSpec for USHFRCOCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcoconf::R`](R) reader structure"]
impl crate::Readable for USHFRCOCONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ushfrcoconf::W`](W) writer structure"]
impl crate::Writable for USHFRCOCONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USHFRCOCONF to value 0x01"]
impl crate::Resettable for USHFRCOCONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
