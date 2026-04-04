#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - Analog Comparator Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Analog Comparator Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUXEN` reader - Input Mux Enable"]
pub type MuxenR = crate::BitReader;
#[doc = "Field `MUXEN` writer - Input Mux Enable"]
pub type MuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTVAL` reader - Inactive Value"]
pub type InactvalR = crate::BitReader;
#[doc = "Field `INACTVAL` writer - Inactive Value"]
pub type InactvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOINV` reader - Comparator GPIO Output Invert"]
pub type GpioinvR = crate::BitReader;
#[doc = "Field `GPIOINV` writer - Comparator GPIO Output Invert"]
pub type GpioinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Hysteresis Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hystsel {
    #[doc = "0: No hysteresis."]
    Hyst0 = 0,
    #[doc = "1: ~15 mV hysteresis."]
    Hyst1 = 1,
    #[doc = "2: ~22 mV hysteresis."]
    Hyst2 = 2,
    #[doc = "3: ~29 mV hysteresis."]
    Hyst3 = 3,
    #[doc = "4: ~36 mV hysteresis."]
    Hyst4 = 4,
    #[doc = "5: ~43 mV hysteresis."]
    Hyst5 = 5,
    #[doc = "6: ~50 mV hysteresis."]
    Hyst6 = 6,
    #[doc = "7: ~57 mV hysteresis."]
    Hyst7 = 7,
}
impl From<Hystsel> for u8 {
    #[inline(always)]
    fn from(variant: Hystsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hystsel {
    type Ux = u8;
}
impl crate::IsEnum for Hystsel {}
#[doc = "Field `HYSTSEL` reader - Hysteresis Select"]
pub type HystselR = crate::FieldReader<Hystsel>;
impl HystselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hystsel {
        match self.bits {
            0 => Hystsel::Hyst0,
            1 => Hystsel::Hyst1,
            2 => Hystsel::Hyst2,
            3 => Hystsel::Hyst3,
            4 => Hystsel::Hyst4,
            5 => Hystsel::Hyst5,
            6 => Hystsel::Hyst6,
            7 => Hystsel::Hyst7,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis."]
    #[inline(always)]
    pub fn is_hyst0(&self) -> bool {
        *self == Hystsel::Hyst0
    }
    #[doc = "~15 mV hysteresis."]
    #[inline(always)]
    pub fn is_hyst1(&self) -> bool {
        *self == Hystsel::Hyst1
    }
    #[doc = "~22 mV hysteresis."]
    #[inline(always)]
    pub fn is_hyst2(&self) -> bool {
        *self == Hystsel::Hyst2
    }
    #[doc = "~29 mV hysteresis."]
    #[inline(always)]
    pub fn is_hyst3(&self) -> bool {
        *self == Hystsel::Hyst3
    }
    #[doc = "~36 mV hysteresis."]
    #[inline(always)]
    pub fn is_hyst4(&self) -> bool {
        *self == Hystsel::Hyst4
    }
    #[doc = "~43 mV hysteresis."]
    #[inline(always)]
    pub fn is_hyst5(&self) -> bool {
        *self == Hystsel::Hyst5
    }
    #[doc = "~50 mV hysteresis."]
    #[inline(always)]
    pub fn is_hyst6(&self) -> bool {
        *self == Hystsel::Hyst6
    }
    #[doc = "~57 mV hysteresis."]
    #[inline(always)]
    pub fn is_hyst7(&self) -> bool {
        *self == Hystsel::Hyst7
    }
}
#[doc = "Field `HYSTSEL` writer - Hysteresis Select"]
pub type HystselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hystsel, crate::Safe>;
impl<'a, REG> HystselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis."]
    #[inline(always)]
    pub fn hyst0(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst0)
    }
    #[doc = "~15 mV hysteresis."]
    #[inline(always)]
    pub fn hyst1(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst1)
    }
    #[doc = "~22 mV hysteresis."]
    #[inline(always)]
    pub fn hyst2(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst2)
    }
    #[doc = "~29 mV hysteresis."]
    #[inline(always)]
    pub fn hyst3(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst3)
    }
    #[doc = "~36 mV hysteresis."]
    #[inline(always)]
    pub fn hyst4(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst4)
    }
    #[doc = "~43 mV hysteresis."]
    #[inline(always)]
    pub fn hyst5(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst5)
    }
    #[doc = "~50 mV hysteresis."]
    #[inline(always)]
    pub fn hyst6(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst6)
    }
    #[doc = "~57 mV hysteresis."]
    #[inline(always)]
    pub fn hyst7(self) -> &'a mut crate::W<REG> {
        self.variant(Hystsel::Hyst7)
    }
}
#[doc = "Warm-up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Warmtime {
    #[doc = "0: 4 HFPERCLK cycles."]
    _4cycles = 0,
    #[doc = "1: 8 HFPERCLK cycles."]
    _8cycles = 1,
    #[doc = "2: 16 HFPERCLK cycles."]
    _16cycles = 2,
    #[doc = "3: 32 HFPERCLK cycles."]
    _32cycles = 3,
    #[doc = "4: 64 HFPERCLK cycles."]
    _64cycles = 4,
    #[doc = "5: 128 HFPERCLK cycles."]
    _128cycles = 5,
    #[doc = "6: 256 HFPERCLK cycles."]
    _256cycles = 6,
    #[doc = "7: 512 HFPERCLK cycles."]
    _512cycles = 7,
}
impl From<Warmtime> for u8 {
    #[inline(always)]
    fn from(variant: Warmtime) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Warmtime {
    type Ux = u8;
}
impl crate::IsEnum for Warmtime {}
#[doc = "Field `WARMTIME` reader - Warm-up Time"]
pub type WarmtimeR = crate::FieldReader<Warmtime>;
impl WarmtimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Warmtime {
        match self.bits {
            0 => Warmtime::_4cycles,
            1 => Warmtime::_8cycles,
            2 => Warmtime::_16cycles,
            3 => Warmtime::_32cycles,
            4 => Warmtime::_64cycles,
            5 => Warmtime::_128cycles,
            6 => Warmtime::_256cycles,
            7 => Warmtime::_512cycles,
            _ => unreachable!(),
        }
    }
    #[doc = "4 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Warmtime::_4cycles
    }
    #[doc = "8 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == Warmtime::_8cycles
    }
    #[doc = "16 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Warmtime::_16cycles
    }
    #[doc = "32 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Warmtime::_32cycles
    }
    #[doc = "64 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Warmtime::_64cycles
    }
    #[doc = "128 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == Warmtime::_128cycles
    }
    #[doc = "256 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Warmtime::_256cycles
    }
    #[doc = "512 HFPERCLK cycles."]
    #[inline(always)]
    pub fn is_512cycles(&self) -> bool {
        *self == Warmtime::_512cycles
    }
}
#[doc = "Field `WARMTIME` writer - Warm-up Time"]
pub type WarmtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Warmtime, crate::Safe>;
impl<'a, REG> WarmtimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_4cycles)
    }
    #[doc = "8 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_8cycles)
    }
    #[doc = "16 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_16cycles)
    }
    #[doc = "32 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_32cycles)
    }
    #[doc = "64 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_64cycles)
    }
    #[doc = "128 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_128cycles)
    }
    #[doc = "256 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_256cycles)
    }
    #[doc = "512 HFPERCLK cycles."]
    #[inline(always)]
    pub fn _512cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_512cycles)
    }
}
#[doc = "Field `IRISE` reader - Rising Edge Interrupt Sense"]
pub type IriseR = crate::BitReader;
#[doc = "Field `IRISE` writer - Rising Edge Interrupt Sense"]
pub type IriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFALL` reader - Falling Edge Interrupt Sense"]
pub type IfallR = crate::BitReader;
#[doc = "Field `IFALL` writer - Falling Edge Interrupt Sense"]
pub type IfallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIASPROG` reader - Bias Configuration"]
pub type BiasprogR = crate::FieldReader;
#[doc = "Field `BIASPROG` writer - Bias Configuration"]
pub type BiasprogW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HalfbiasR = crate::BitReader;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HalfbiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLBIAS` reader - Full Bias Current"]
pub type FullbiasR = crate::BitReader;
#[doc = "Field `FULLBIAS` writer - Full Bias Current"]
pub type FullbiasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Mux Enable"]
    #[inline(always)]
    pub fn muxen(&self) -> MuxenR {
        MuxenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> InactvalR {
        InactvalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&self) -> GpioinvR {
        GpioinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Hysteresis Select"]
    #[inline(always)]
    pub fn hystsel(&self) -> HystselR {
        HystselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Warm-up Time"]
    #[inline(always)]
    pub fn warmtime(&self) -> WarmtimeR {
        WarmtimeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&self) -> IriseR {
        IriseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&self) -> IfallR {
        IfallR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&self) -> BiasprogR {
        BiasprogR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HalfbiasR {
        HalfbiasR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&self) -> FullbiasR {
        FullbiasR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Mux Enable"]
    #[inline(always)]
    pub fn muxen(&mut self) -> MuxenW<'_, CtrlSpec> {
        MuxenW::new(self, 1)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&mut self) -> InactvalW<'_, CtrlSpec> {
        InactvalW::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&mut self) -> GpioinvW<'_, CtrlSpec> {
        GpioinvW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Hysteresis Select"]
    #[inline(always)]
    pub fn hystsel(&mut self) -> HystselW<'_, CtrlSpec> {
        HystselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Warm-up Time"]
    #[inline(always)]
    pub fn warmtime(&mut self) -> WarmtimeW<'_, CtrlSpec> {
        WarmtimeW::new(self, 8)
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&mut self) -> IriseW<'_, CtrlSpec> {
        IriseW::new(self, 16)
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&mut self) -> IfallW<'_, CtrlSpec> {
        IfallW::new(self, 17)
    }
    #[doc = "Bits 24:27 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BiasprogW<'_, CtrlSpec> {
        BiasprogW::new(self, 24)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HalfbiasW<'_, CtrlSpec> {
        HalfbiasW::new(self, 30)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&mut self) -> FullbiasW<'_, CtrlSpec> {
        FullbiasW::new(self, 31)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x4700_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x4700_0000;
}
