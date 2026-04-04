#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfclksel {
    #[doc = "1: Select HFRCO as HFCLK."]
    Hfrco = 1,
    #[doc = "2: Select HFXO as HFCLK."]
    Hfxo = 2,
    #[doc = "3: Select LFRCO as HFCLK."]
    Lfrco = 3,
    #[doc = "4: Select LFXO as HFCLK."]
    Lfxo = 4,
    #[doc = "5: Select USHFRCO divided by two as HFCLK."]
    Ushfrcodiv2 = 5,
}
impl From<Hfclksel> for u8 {
    #[inline(always)]
    fn from(variant: Hfclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfclksel {
    type Ux = u8;
}
impl crate::IsEnum for Hfclksel {}
#[doc = "Field `HFCLKSEL` writer - HFCLK Select"]
pub type HfclkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hfclksel>;
impl<'a, REG> HfclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFRCO as HFCLK."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclksel::Hfrco)
    }
    #[doc = "Select HFXO as HFCLK."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclksel::Hfxo)
    }
    #[doc = "Select LFRCO as HFCLK."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclksel::Lfrco)
    }
    #[doc = "Select LFXO as HFCLK."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclksel::Lfxo)
    }
    #[doc = "Select USHFRCO divided by two as HFCLK."]
    #[inline(always)]
    pub fn ushfrcodiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfclksel::Ushfrcodiv2)
    }
}
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub type CalstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CalstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB Core Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbcclksel {
    #[doc = "2: Select LFXO as HFCORECLKUSBC."]
    Lfxo = 2,
    #[doc = "3: Select LFRCO as HFCORECLKUSBC."]
    Lfrco = 3,
    #[doc = "4: Select USHFRCO as HFCORECLKUSBC."]
    Ushfrco = 4,
}
impl From<Usbcclksel> for u8 {
    #[inline(always)]
    fn from(variant: Usbcclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbcclksel {
    type Ux = u8;
}
impl crate::IsEnum for Usbcclksel {}
#[doc = "Field `USBCCLKSEL` writer - USB Core Clock Select"]
pub type UsbcclkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usbcclksel>;
impl<'a, REG> UsbcclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select LFXO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcclksel::Lfxo)
    }
    #[doc = "Select LFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcclksel::Lfrco)
    }
    #[doc = "Select USHFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcclksel::Ushfrco)
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    pub fn hfclksel(&mut self) -> HfclkselW<'_, CmdSpec> {
        HfclkselW::new(self, 0)
    }
    #[doc = "Bit 3 - Calibration Start"]
    #[inline(always)]
    pub fn calstart(&mut self) -> CalstartW<'_, CmdSpec> {
        CalstartW::new(self, 3)
    }
    #[doc = "Bit 4 - Calibration Stop"]
    #[inline(always)]
    pub fn calstop(&mut self) -> CalstopW<'_, CmdSpec> {
        CalstopW::new(self, 4)
    }
    #[doc = "Bits 5:7 - USB Core Clock Select"]
    #[inline(always)]
    pub fn usbcclksel(&mut self) -> UsbcclkselW<'_, CmdSpec> {
        UsbcclkselW::new(self, 5)
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
