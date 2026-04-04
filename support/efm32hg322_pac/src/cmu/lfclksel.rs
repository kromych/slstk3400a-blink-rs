#[doc = "Register `LFCLKSEL` reader"]
pub type R = crate::R<LfclkselSpec>;
#[doc = "Register `LFCLKSEL` writer"]
pub type W = crate::W<LfclkselSpec>;
#[doc = "Clock Select for LFA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfa {
    #[doc = "0: LFACLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    Lfxo = 2,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    Hfcoreclklediv2 = 3,
}
impl From<Lfa> for u8 {
    #[inline(always)]
    fn from(variant: Lfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfa {
    type Ux = u8;
}
impl crate::IsEnum for Lfa {}
#[doc = "Field `LFA` reader - Clock Select for LFA"]
pub type LfaR = crate::FieldReader<Lfa>;
impl LfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfa {
        match self.bits {
            0 => Lfa::Disabled,
            1 => Lfa::Lfrco,
            2 => Lfa::Lfxo,
            3 => Lfa::Hfcoreclklediv2,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfa::Disabled
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Lfa::Lfrco
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Lfa::Lfxo
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == Lfa::Hfcoreclklediv2
    }
}
#[doc = "Field `LFA` writer - Clock Select for LFA"]
pub type LfaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfa, crate::Safe>;
impl<'a, REG> LfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Disabled)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Lfrco)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Lfxo)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut crate::W<REG> {
        self.variant(Lfa::Hfcoreclklediv2)
    }
}
#[doc = "Clock Select for LFB\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfb {
    #[doc = "0: LFBCLK is disabled"]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    Lfxo = 2,
    #[doc = "3: HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    Hfcoreclklediv2 = 3,
}
impl From<Lfb> for u8 {
    #[inline(always)]
    fn from(variant: Lfb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfb {
    type Ux = u8;
}
impl crate::IsEnum for Lfb {}
#[doc = "Field `LFB` reader - Clock Select for LFB"]
pub type LfbR = crate::FieldReader<Lfb>;
impl LfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfb {
        match self.bits {
            0 => Lfb::Disabled,
            1 => Lfb::Lfrco,
            2 => Lfb::Lfxo,
            3 => Lfb::Hfcoreclklediv2,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfb::Disabled
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Lfb::Lfrco
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Lfb::Lfxo
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn is_hfcoreclklediv2(&self) -> bool {
        *self == Lfb::Hfcoreclklediv2
    }
}
#[doc = "Field `LFB` writer - Clock Select for LFB"]
pub type LfbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfb, crate::Safe>;
impl<'a, REG> LfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Disabled)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Lfrco)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Lfxo)
    }
    #[doc = "HFCORECLKLE divided by two or four is selected as LFACLK. The division factor is determined by CMU_CTRL_HFLE and CMU_HFCORECLKDIV_HFCORECLKLEDIV."]
    #[inline(always)]
    pub fn hfcoreclklediv2(self) -> &'a mut crate::W<REG> {
        self.variant(Lfb::Hfcoreclklediv2)
    }
}
#[doc = "Clock Select for LFC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfc {
    #[doc = "0: LFCCLK clock disabled."]
    Disabled = 0,
    #[doc = "1: LFRCO selected as LFCCLK clock"]
    Lfrco = 1,
    #[doc = "2: LFXO selected as LFCCLK clock"]
    Lfxo = 2,
}
impl From<Lfc> for u8 {
    #[inline(always)]
    fn from(variant: Lfc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfc {
    type Ux = u8;
}
impl crate::IsEnum for Lfc {}
#[doc = "Field `LFC` reader - Clock Select for LFC"]
pub type LfcR = crate::FieldReader<Lfc>;
impl LfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfc> {
        match self.bits {
            0 => Some(Lfc::Disabled),
            1 => Some(Lfc::Lfrco),
            2 => Some(Lfc::Lfxo),
            _ => None,
        }
    }
    #[doc = "LFCCLK clock disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfc::Disabled
    }
    #[doc = "LFRCO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Lfc::Lfrco
    }
    #[doc = "LFXO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Lfc::Lfxo
    }
}
#[doc = "Field `LFC` writer - Clock Select for LFC"]
pub type LfcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfc>;
impl<'a, REG> LfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFCCLK clock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lfc::Disabled)
    }
    #[doc = "LFRCO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lfc::Lfrco)
    }
    #[doc = "LFXO selected as LFCCLK clock"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Lfc::Lfxo)
    }
}
#[doc = "Field `LFAE` reader - Clock Select for LFA Extended"]
pub type LfaeR = crate::BitReader;
#[doc = "Field `LFAE` writer - Clock Select for LFA Extended"]
pub type LfaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFBE` reader - Clock Select for LFB Extended"]
pub type LfbeR = crate::BitReader;
#[doc = "Field `LFBE` writer - Clock Select for LFB Extended"]
pub type LfbeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LfaR {
        LfaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LfbR {
        LfbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&self) -> LfcR {
        LfcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    pub fn lfae(&self) -> LfaeR {
        LfaeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    pub fn lfbe(&self) -> LfbeR {
        LfbeR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&mut self) -> LfaW<'_, LfclkselSpec> {
        LfaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&mut self) -> LfbW<'_, LfclkselSpec> {
        LfbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&mut self) -> LfcW<'_, LfclkselSpec> {
        LfcW::new(self, 4)
    }
    #[doc = "Bit 16 - Clock Select for LFA Extended"]
    #[inline(always)]
    pub fn lfae(&mut self) -> LfaeW<'_, LfclkselSpec> {
        LfaeW::new(self, 16)
    }
    #[doc = "Bit 20 - Clock Select for LFB Extended"]
    #[inline(always)]
    pub fn lfbe(&mut self) -> LfbeW<'_, LfclkselSpec> {
        LfbeW::new(self, 20)
    }
}
#[doc = "Low Frequency Clock Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclkselSpec;
impl crate::RegisterSpec for LfclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksel::R`](R) reader structure"]
impl crate::Readable for LfclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`lfclksel::W`](W) writer structure"]
impl crate::Writable for LfclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFCLKSEL to value 0x15"]
impl crate::Resettable for LfclkselSpec {
    const RESET_VALUE: u32 = 0x15;
}
