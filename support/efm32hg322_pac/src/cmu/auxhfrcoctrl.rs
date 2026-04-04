#[doc = "Register `AUXHFRCOCTRL` reader"]
pub type R = crate::R<AuxhfrcoctrlSpec>;
#[doc = "Register `AUXHFRCOCTRL` writer"]
pub type W = crate::W<AuxhfrcoctrlSpec>;
#[doc = "Field `TUNING` reader - AUXHFRCO Tuning Value"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - AUXHFRCO Tuning Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "AUXHFRCO Band Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Band {
    #[doc = "0: 14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14mhz = 0,
    #[doc = "1: 11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11mhz = 1,
    #[doc = "2: 7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7mhz = 2,
    #[doc = "3: 1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1mhz = 3,
    #[doc = "7: 21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21mhz = 7,
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
#[doc = "Field `BAND` reader - AUXHFRCO Band Select"]
pub type BandR = crate::FieldReader<Band>;
impl BandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Band> {
        match self.bits {
            0 => Some(Band::_14mhz),
            1 => Some(Band::_11mhz),
            2 => Some(Band::_7mhz),
            3 => Some(Band::_1mhz),
            7 => Some(Band::_21mhz),
            _ => None,
        }
    }
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_14mhz(&self) -> bool {
        *self == Band::_14mhz
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_11mhz(&self) -> bool {
        *self == Band::_11mhz
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_7mhz(&self) -> bool {
        *self == Band::_7mhz
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == Band::_1mhz
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn is_21mhz(&self) -> bool {
        *self == Band::_21mhz
    }
}
#[doc = "Field `BAND` writer - AUXHFRCO Band Select"]
pub type BandW<'a, REG> = crate::FieldWriter<'a, REG, 3, Band>;
impl<'a, REG> BandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _14mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_14mhz)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _11mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_11mhz)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _7mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_7mhz)
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_1mhz)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _21mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Band::_21mhz)
    }
}
impl R {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BandR {
        BandR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, AuxhfrcoctrlSpec> {
        TuningW::new(self, 0)
    }
    #[doc = "Bits 8:10 - AUXHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BandW<'_, AuxhfrcoctrlSpec> {
        BandW::new(self, 8)
    }
}
#[doc = "AUXHFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`auxhfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auxhfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxhfrcoctrlSpec;
impl crate::RegisterSpec for AuxhfrcoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxhfrcoctrl::R`](R) reader structure"]
impl crate::Readable for AuxhfrcoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`auxhfrcoctrl::W`](W) writer structure"]
impl crate::Writable for AuxhfrcoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUXHFRCOCTRL to value 0x80"]
impl crate::Resettable for AuxhfrcoctrlSpec {
    const RESET_VALUE: u32 = 0x80;
}
