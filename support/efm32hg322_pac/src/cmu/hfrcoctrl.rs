#[doc = "Register `HFRCOCTRL` reader"]
pub type R = crate::R<HfrcoctrlSpec>;
#[doc = "Register `HFRCOCTRL` writer"]
pub type W = crate::W<HfrcoctrlSpec>;
#[doc = "Field `TUNING` reader - HFRCO Tuning Value"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - HFRCO Tuning Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "HFRCO Band Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Band {
    #[doc = "0: 1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1mhz = 0,
    #[doc = "1: 7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7mhz = 1,
    #[doc = "2: 11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11mhz = 2,
    #[doc = "3: 14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14mhz = 3,
    #[doc = "4: 21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21mhz = 4,
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
#[doc = "Field `BAND` reader - HFRCO Band Select"]
pub type BandR = crate::FieldReader<Band>;
impl BandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Band> {
        match self.bits {
            0 => Some(Band::_1mhz),
            1 => Some(Band::_7mhz),
            2 => Some(Band::_11mhz),
            3 => Some(Band::_14mhz),
            4 => Some(Band::_21mhz),
            _ => None,
        }
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == Band::_1mhz
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_7mhz(&self) -> bool {
        *self == Band::_7mhz
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_11mhz(&self) -> bool {
        *self == Band::_11mhz
    }
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_14mhz(&self) -> bool {
        *self == Band::_14mhz
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_21mhz(&self) -> bool {
        *self == Band::_21mhz
    }
}
#[doc = "Field `BAND` writer - HFRCO Band Select"]
pub type BandW<'a, REG> = crate::FieldWriter<'a, REG, 3, Band>;
impl<'a, REG> BandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_1mhz)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _7mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_7mhz)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _11mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_11mhz)
    }
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _14mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_14mhz)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _21mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_21mhz)
    }
}
#[doc = "Field `SUDELAY` reader - HFRCO Start-up Delay"]
pub type SudelayR = crate::FieldReader;
#[doc = "Field `SUDELAY` writer - HFRCO Start-up Delay"]
pub type SudelayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BandR {
        BandR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline(always)]
    pub fn sudelay(&self) -> SudelayR {
        SudelayR::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, HfrcoctrlSpec> {
        TuningW::new(self, 0)
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BandW<'_, HfrcoctrlSpec> {
        BandW::new(self, 8)
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline(always)]
    pub fn sudelay(&mut self) -> SudelayW<'_, HfrcoctrlSpec> {
        SudelayW::new(self, 12)
    }
}
#[doc = "HFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfrcoctrlSpec;
impl crate::RegisterSpec for HfrcoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrcoctrl::R`](R) reader structure"]
impl crate::Readable for HfrcoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfrcoctrl::W`](W) writer structure"]
impl crate::Writable for HfrcoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFRCOCTRL to value 0x0380"]
impl crate::Resettable for HfrcoctrlSpec {
    const RESET_VALUE: u32 = 0x0380;
}
