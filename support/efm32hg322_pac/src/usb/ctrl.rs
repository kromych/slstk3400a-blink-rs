#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `DMPUAP` reader - DMPU Active Polarity"]
pub type DMPUAP_R = crate::BitReader;
#[doc = "Field `DMPUAP` writer - DMPU Active Polarity"]
pub type DMPUAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMOSCCTRL` reader - Low Energy Mode Oscillator Control"]
pub type LEMOSCCTRL_R = crate::FieldReader<LEMOSCCTRL_A>;
#[doc = "Low Energy Mode Oscillator Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEMOSCCTRL_A {
    #[doc = "0: Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE = 0,
    #[doc = "1: The USBC clock is gated when Low Energy Mode is active."]
    GATE = 1,
    #[doc = "2: The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    SUSPEND = 2,
}
impl From<LEMOSCCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEMOSCCTRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEMOSCCTRL_A {
    type Ux = u8;
}
impl LEMOSCCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LEMOSCCTRL_A> {
        match self.bits {
            0 => Some(LEMOSCCTRL_A::NONE),
            1 => Some(LEMOSCCTRL_A::GATE),
            2 => Some(LEMOSCCTRL_A::SUSPEND),
            _ => None,
        }
    }
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LEMOSCCTRL_A::NONE
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == LEMOSCCTRL_A::GATE
    }
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == LEMOSCCTRL_A::SUSPEND
    }
}
#[doc = "Field `LEMOSCCTRL` writer - Low Energy Mode Oscillator Control"]
pub type LEMOSCCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LEMOSCCTRL_A>;
impl<'a, REG> LEMOSCCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(LEMOSCCTRL_A::NONE)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut crate::W<REG> {
        self.variant(LEMOSCCTRL_A::GATE)
    }
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(LEMOSCCTRL_A::SUSPEND)
    }
}
#[doc = "Field `LEMPHYCTRL` reader - Low Energy Mode USB PHY Control"]
pub type LEMPHYCTRL_R = crate::BitReader;
#[doc = "Field `LEMPHYCTRL` writer - Low Energy Mode USB PHY Control"]
pub type LEMPHYCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMIDLEEN` reader - Low Energy Mode on Bus Idle Enable"]
pub type LEMIDLEEN_R = crate::BitReader;
#[doc = "Field `LEMIDLEEN` writer - Low Energy Mode on Bus Idle Enable"]
pub type LEMIDLEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMNAKEN` reader - Low Energy Mode on OUT NAK Enable"]
pub type LEMNAKEN_R = crate::BitReader;
#[doc = "Field `LEMNAKEN` writer - Low Energy Mode on OUT NAK Enable"]
pub type LEMNAKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEMADDRMEN` reader - Low Energy Mode on Device Address Mismatch Enable"]
pub type LEMADDRMEN_R = crate::BitReader;
#[doc = "Field `LEMADDRMEN` writer - Low Energy Mode on Device Address Mismatch Enable"]
pub type LEMADDRMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGDIS` reader - Voltage Regulator Disable"]
pub type VREGDIS_R = crate::BitReader;
#[doc = "Field `VREGDIS` writer - Voltage Regulator Disable"]
pub type VREGDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGOSEN` reader - VREGO Sense Enable"]
pub type VREGOSEN_R = crate::BitReader;
#[doc = "Field `VREGOSEN` writer - VREGO Sense Enable"]
pub type VREGOSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIASPROGEM01` reader - Regulator Bias Programming Value in EM0/1"]
pub type BIASPROGEM01_R = crate::FieldReader;
#[doc = "Field `BIASPROGEM01` writer - Regulator Bias Programming Value in EM0/1"]
pub type BIASPROGEM01_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIASPROGEM23` reader - Regulator Bias Programming Value in EM2/3"]
pub type BIASPROGEM23_R = crate::FieldReader;
#[doc = "Field `BIASPROGEM23` writer - Regulator Bias Programming Value in EM2/3"]
pub type BIASPROGEM23_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&self) -> DMPUAP_R {
        DMPUAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&self) -> LEMOSCCTRL_R {
        LEMOSCCTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&self) -> LEMPHYCTRL_R {
        LEMPHYCTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&self) -> LEMIDLEEN_R {
        LEMIDLEEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    pub fn lemnaken(&self) -> LEMNAKEN_R {
        LEMNAKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    pub fn lemaddrmen(&self) -> LEMADDRMEN_R {
        LEMADDRMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&self) -> VREGDIS_R {
        VREGDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&self) -> VREGOSEN_R {
        VREGOSEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&self) -> BIASPROGEM01_R {
        BIASPROGEM01_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&self) -> BIASPROGEM23_R {
        BIASPROGEM23_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dmpuap(&mut self) -> DMPUAP_W<CTRL_SPEC> {
        DMPUAP_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    #[must_use]
    pub fn lemoscctrl(&mut self) -> LEMOSCCTRL_W<CTRL_SPEC> {
        LEMOSCCTRL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    #[must_use]
    pub fn lemphyctrl(&mut self) -> LEMPHYCTRL_W<CTRL_SPEC> {
        LEMPHYCTRL_W::new(self, 7)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lemidleen(&mut self) -> LEMIDLEEN_W<CTRL_SPEC> {
        LEMIDLEEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lemnaken(&mut self) -> LEMNAKEN_W<CTRL_SPEC> {
        LEMNAKEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lemaddrmen(&mut self) -> LEMADDRMEN_W<CTRL_SPEC> {
        LEMADDRMEN_W::new(self, 11)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vregdis(&mut self) -> VREGDIS_W<CTRL_SPEC> {
        VREGDIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vregosen(&mut self) -> VREGOSEN_W<CTRL_SPEC> {
        VREGOSEN_W::new(self, 17)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    #[must_use]
    pub fn biasprogem01(&mut self) -> BIASPROGEM01_W<CTRL_SPEC> {
        BIASPROGEM01_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    #[must_use]
    pub fn biasprogem23(&mut self) -> BIASPROGEM23_W<CTRL_SPEC> {
        BIASPROGEM23_W::new(self, 24)
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
#[doc = "System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x20"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
