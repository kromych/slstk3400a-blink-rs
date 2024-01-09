#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFCLKSEL_AW {
    #[doc = "1: Select HFRCO as HFCLK."]
    HFRCO = 1,
    #[doc = "2: Select HFXO as HFCLK."]
    HFXO = 2,
    #[doc = "3: Select LFRCO as HFCLK."]
    LFRCO = 3,
    #[doc = "4: Select LFXO as HFCLK."]
    LFXO = 4,
    #[doc = "5: Select USHFRCO divided by two as HFCLK."]
    USHFRCODIV2 = 5,
}
impl From<HFCLKSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: HFCLKSEL_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HFCLKSEL_AW {
    type Ux = u8;
}
#[doc = "Field `HFCLKSEL` writer - HFCLK Select"]
pub type HFCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, HFCLKSEL_AW>;
impl<'a, REG> HFCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFRCO as HFCLK."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKSEL_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKSEL_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKSEL_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKSEL_AW::LFXO)
    }
    #[doc = "Select USHFRCO divided by two as HFCLK."]
    #[inline(always)]
    pub fn ushfrcodiv2(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKSEL_AW::USHFRCODIV2)
    }
}
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub type CALSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CALSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB Core Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBCCLKSEL_AW {
    #[doc = "2: Select LFXO as HFCORECLKUSBC."]
    LFXO = 2,
    #[doc = "3: Select LFRCO as HFCORECLKUSBC."]
    LFRCO = 3,
    #[doc = "4: Select USHFRCO as HFCORECLKUSBC."]
    USHFRCO = 4,
}
impl From<USBCCLKSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: USBCCLKSEL_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBCCLKSEL_AW {
    type Ux = u8;
}
#[doc = "Field `USBCCLKSEL` writer - USB Core Clock Select"]
pub type USBCCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USBCCLKSEL_AW>;
impl<'a, REG> USBCCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select LFXO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(USBCCLKSEL_AW::LFXO)
    }
    #[doc = "Select LFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(USBCCLKSEL_AW::LFRCO)
    }
    #[doc = "Select USHFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(USBCCLKSEL_AW::USHFRCO)
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn hfclksel(&mut self) -> HFCLKSEL_W<CMD_SPEC> {
        HFCLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn calstart(&mut self) -> CALSTART_W<CMD_SPEC> {
        CALSTART_W::new(self, 3)
    }
    #[doc = "Bit 4 - Calibration Stop"]
    #[inline(always)]
    #[must_use]
    pub fn calstop(&mut self) -> CALSTOP_W<CMD_SPEC> {
        CALSTOP_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - USB Core Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbcclksel(&mut self) -> USBCCLKSEL_W<CMD_SPEC> {
        USBCCLKSEL_W::new(self, 5)
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
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
