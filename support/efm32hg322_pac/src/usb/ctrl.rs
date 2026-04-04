#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `DMPUAP` reader - DMPU Active Polarity"]
pub type DmpuapR = crate::BitReader;
#[doc = "Field `DMPUAP` writer - DMPU Active Polarity"]
pub type DmpuapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low Energy Mode Oscillator Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lemoscctrl {
    #[doc = "0: Low Energy Mode has no effect on neither USBC or USHFRCO."]
    None = 0,
    #[doc = "1: The USBC clock is gated when Low Energy Mode is active."]
    Gate = 1,
    #[doc = "2: The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    Suspend = 2,
}
impl From<Lemoscctrl> for u8 {
    #[inline(always)]
    fn from(variant: Lemoscctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lemoscctrl {
    type Ux = u8;
}
impl crate::IsEnum for Lemoscctrl {}
#[doc = "Field `LEMOSCCTRL` reader - Low Energy Mode Oscillator Control"]
pub type LemoscctrlR = crate::FieldReader<Lemoscctrl>;
impl LemoscctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lemoscctrl> {
        match self.bits {
            0 => Some(Lemoscctrl::None),
            1 => Some(Lemoscctrl::Gate),
            2 => Some(Lemoscctrl::Suspend),
            _ => None,
        }
    }
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Lemoscctrl::None
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == Lemoscctrl::Gate
    }
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Lemoscctrl::Suspend
    }
}
#[doc = "Field `LEMOSCCTRL` writer - Low Energy Mode Oscillator Control"]
pub type LemoscctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lemoscctrl>;
impl<'a, REG> LemoscctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Lemoscctrl::None)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut crate::W<REG> {
        self.variant(Lemoscctrl::Gate)
    }
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Lemoscctrl::Suspend)
    }
}
#[doc = "Field `LEMPHYCTRL` reader - Low Energy Mode USB PHY Control"]
pub type LemphyctrlR = crate::BitReader;
#[doc = "Field `LEMPHYCTRL` writer - Low Energy Mode USB PHY Control"]
pub type LemphyctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMIDLEEN` reader - Low Energy Mode on Bus Idle Enable"]
pub type LemidleenR = crate::BitReader;
#[doc = "Field `LEMIDLEEN` writer - Low Energy Mode on Bus Idle Enable"]
pub type LemidleenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMNAKEN` reader - Low Energy Mode on OUT NAK Enable"]
pub type LemnakenR = crate::BitReader;
#[doc = "Field `LEMNAKEN` writer - Low Energy Mode on OUT NAK Enable"]
pub type LemnakenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMADDRMEN` reader - Low Energy Mode on Device Address Mismatch Enable"]
pub type LemaddrmenR = crate::BitReader;
#[doc = "Field `LEMADDRMEN` writer - Low Energy Mode on Device Address Mismatch Enable"]
pub type LemaddrmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGDIS` reader - Voltage Regulator Disable"]
pub type VregdisR = crate::BitReader;
#[doc = "Field `VREGDIS` writer - Voltage Regulator Disable"]
pub type VregdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGOSEN` reader - VREGO Sense Enable"]
pub type VregosenR = crate::BitReader;
#[doc = "Field `VREGOSEN` writer - VREGO Sense Enable"]
pub type VregosenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIASPROGEM01` reader - Regulator Bias Programming Value in EM0/1"]
pub type Biasprogem01R = crate::FieldReader;
#[doc = "Field `BIASPROGEM01` writer - Regulator Bias Programming Value in EM0/1"]
pub type Biasprogem01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIASPROGEM23` reader - Regulator Bias Programming Value in EM2/3"]
pub type Biasprogem23R = crate::FieldReader;
#[doc = "Field `BIASPROGEM23` writer - Regulator Bias Programming Value in EM2/3"]
pub type Biasprogem23W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&self) -> DmpuapR {
        DmpuapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&self) -> LemoscctrlR {
        LemoscctrlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&self) -> LemphyctrlR {
        LemphyctrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&self) -> LemidleenR {
        LemidleenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    pub fn lemnaken(&self) -> LemnakenR {
        LemnakenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    pub fn lemaddrmen(&self) -> LemaddrmenR {
        LemaddrmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&self) -> VregdisR {
        VregdisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&self) -> VregosenR {
        VregosenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&self) -> Biasprogem01R {
        Biasprogem01R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&self) -> Biasprogem23R {
        Biasprogem23R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&mut self) -> DmpuapW<'_, CtrlSpec> {
        DmpuapW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&mut self) -> LemoscctrlW<'_, CtrlSpec> {
        LemoscctrlW::new(self, 4)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&mut self) -> LemphyctrlW<'_, CtrlSpec> {
        LemphyctrlW::new(self, 7)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&mut self) -> LemidleenW<'_, CtrlSpec> {
        LemidleenW::new(self, 9)
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    pub fn lemnaken(&mut self) -> LemnakenW<'_, CtrlSpec> {
        LemnakenW::new(self, 10)
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    pub fn lemaddrmen(&mut self) -> LemaddrmenW<'_, CtrlSpec> {
        LemaddrmenW::new(self, 11)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&mut self) -> VregdisW<'_, CtrlSpec> {
        VregdisW::new(self, 16)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&mut self) -> VregosenW<'_, CtrlSpec> {
        VregosenW::new(self, 17)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&mut self) -> Biasprogem01W<'_, CtrlSpec> {
        Biasprogem01W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&mut self) -> Biasprogem23W<'_, CtrlSpec> {
        Biasprogem23W::new(self, 24)
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x20"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x20;
}
