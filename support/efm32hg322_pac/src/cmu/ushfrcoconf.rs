#[doc = "Register `USHFRCOCONF` reader"]
pub type R = crate::R<UshfrcoconfSpec>;
#[doc = "Register `USHFRCOCONF` writer"]
pub type W = crate::W<UshfrcoconfSpec>;
#[doc = "USHFRCO Band Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Band {
    #[doc = "1: 48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _48mhz = 1,
    #[doc = "3: 24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _24mhz = 3,
}
impl From<Band> for u8 {
    #[inline(always)]
    fn from(variant: Band) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Band {
    type Ux = u8;
}
impl crate::IsEnum for Band {}
#[doc = "Field `BAND` reader - USHFRCO Band Select"]
pub type BandR = crate::FieldReader<Band>;
impl BandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Band> {
        match self.bits {
            1 => Some(Band::_48mhz),
            3 => Some(Band::_24mhz),
            _ => None,
        }
    }
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn is_48mhz(&self) -> bool {
        *self == Band::_48mhz
    }
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == Band::_24mhz
    }
}
#[doc = "Field `BAND` writer - USHFRCO Band Select"]
pub type BandW<'a, REG> = crate::FieldWriter<'a, REG, 3, Band>;
impl<'a, REG> BandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _48mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_48mhz)
    }
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_24mhz)
    }
}
#[doc = "Field `USHFRCODIV2DIS` reader - USHFRCO divider for HFCLK disable"]
pub type Ushfrcodiv2disR = crate::BitReader;
#[doc = "Field `USHFRCODIV2DIS` writer - USHFRCO divider for HFCLK disable"]
pub type Ushfrcodiv2disW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BandR {
        BandR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&self) -> Ushfrcodiv2disR {
        Ushfrcodiv2disR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BandW<'_, UshfrcoconfSpec> {
        BandW::new(self, 0)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&mut self) -> Ushfrcodiv2disW<'_, UshfrcoconfSpec> {
        Ushfrcodiv2disW::new(self, 4)
    }
}
#[doc = "USHFRCO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcoconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ushfrcoconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UshfrcoconfSpec;
impl crate::RegisterSpec for UshfrcoconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcoconf::R`](R) reader structure"]
impl crate::Readable for UshfrcoconfSpec {}
#[doc = "`write(|w| ..)` method takes [`ushfrcoconf::W`](W) writer structure"]
impl crate::Writable for UshfrcoconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USHFRCOCONF to value 0x01"]
impl crate::Resettable for UshfrcoconfSpec {
    const RESET_VALUE: u32 = 0x01;
}
