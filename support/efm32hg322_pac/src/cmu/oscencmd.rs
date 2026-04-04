#[doc = "Register `OSCENCMD` writer"]
pub type W = crate::W<OscencmdSpec>;
#[doc = "Field `HFRCOEN` writer - HFRCO Enable"]
pub type HfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - HFRCO Disable"]
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOEN` writer - HFXO Enable"]
pub type HfxoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODIS` writer - HFXO Disable"]
pub type HfxodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCOEN` writer - AUXHFRCO Enable"]
pub type AuxhfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCODIS` writer - AUXHFRCO Disable"]
pub type AuxhfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEN` writer - LFRCO Enable"]
pub type LfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCODIS` writer - LFRCO Disable"]
pub type LfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEN` writer - LFXO Enable"]
pub type LfxoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXODIS` writer - LFXO Disable"]
pub type LfxodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCOEN` writer - USHFRCO Enable"]
pub type UshfrcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCODIS` writer - USHFRCO Disable"]
pub type UshfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    pub fn hfrcoen(&mut self) -> HfrcoenW<'_, OscencmdSpec> {
        HfrcoenW::new(self, 0)
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HfrcodisW<'_, OscencmdSpec> {
        HfrcodisW::new(self, 1)
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    pub fn hfxoen(&mut self) -> HfxoenW<'_, OscencmdSpec> {
        HfxoenW::new(self, 2)
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    pub fn hfxodis(&mut self) -> HfxodisW<'_, OscencmdSpec> {
        HfxodisW::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    pub fn auxhfrcoen(&mut self) -> AuxhfrcoenW<'_, OscencmdSpec> {
        AuxhfrcoenW::new(self, 4)
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    pub fn auxhfrcodis(&mut self) -> AuxhfrcodisW<'_, OscencmdSpec> {
        AuxhfrcodisW::new(self, 5)
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    pub fn lfrcoen(&mut self) -> LfrcoenW<'_, OscencmdSpec> {
        LfrcoenW::new(self, 6)
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    pub fn lfrcodis(&mut self) -> LfrcodisW<'_, OscencmdSpec> {
        LfrcodisW::new(self, 7)
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    pub fn lfxoen(&mut self) -> LfxoenW<'_, OscencmdSpec> {
        LfxoenW::new(self, 8)
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    pub fn lfxodis(&mut self) -> LfxodisW<'_, OscencmdSpec> {
        LfxodisW::new(self, 9)
    }
    #[doc = "Bit 10 - USHFRCO Enable"]
    #[inline(always)]
    pub fn ushfrcoen(&mut self) -> UshfrcoenW<'_, OscencmdSpec> {
        UshfrcoenW::new(self, 10)
    }
    #[doc = "Bit 11 - USHFRCO Disable"]
    #[inline(always)]
    pub fn ushfrcodis(&mut self) -> UshfrcodisW<'_, OscencmdSpec> {
        UshfrcodisW::new(self, 11)
    }
}
#[doc = "Oscillator Enable/Disable Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscencmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscencmdSpec;
impl crate::RegisterSpec for OscencmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oscencmd::W`](W) writer structure"]
impl crate::Writable for OscencmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSCENCMD to value 0"]
impl crate::Resettable for OscencmdSpec {}
