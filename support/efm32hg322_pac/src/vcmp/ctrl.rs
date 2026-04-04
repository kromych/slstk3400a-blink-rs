#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - Voltage Supply Comparator Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Voltage Supply Comparator Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTVAL` reader - Inactive Value"]
pub type InactvalR = crate::BitReader;
#[doc = "Field `INACTVAL` writer - Inactive Value"]
pub type InactvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTEN` reader - Hysteresis Enable"]
pub type HystenR = crate::BitReader;
#[doc = "Field `HYSTEN` writer - Hysteresis Enable"]
pub type HystenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Warm-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Warmtime {
    #[doc = "0: 4 HFPERCLK cycles"]
    _4cycles = 0,
    #[doc = "1: 8 HFPERCLK cycles"]
    _8cycles = 1,
    #[doc = "2: 16 HFPERCLK cycles"]
    _16cycles = 2,
    #[doc = "3: 32 HFPERCLK cycles"]
    _32cycles = 3,
    #[doc = "4: 64 HFPERCLK cycles"]
    _64cycles = 4,
    #[doc = "5: 128 HFPERCLK cycles"]
    _128cycles = 5,
    #[doc = "6: 256 HFPERCLK cycles"]
    _256cycles = 6,
    #[doc = "7: 512 HFPERCLK cycles"]
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
#[doc = "Field `WARMTIME` reader - Warm-Up Time"]
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
    #[doc = "4 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Warmtime::_4cycles
    }
    #[doc = "8 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == Warmtime::_8cycles
    }
    #[doc = "16 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Warmtime::_16cycles
    }
    #[doc = "32 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Warmtime::_32cycles
    }
    #[doc = "64 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Warmtime::_64cycles
    }
    #[doc = "128 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == Warmtime::_128cycles
    }
    #[doc = "256 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Warmtime::_256cycles
    }
    #[doc = "512 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_512cycles(&self) -> bool {
        *self == Warmtime::_512cycles
    }
}
#[doc = "Field `WARMTIME` writer - Warm-Up Time"]
pub type WarmtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Warmtime, crate::Safe>;
impl<'a, REG> WarmtimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_4cycles)
    }
    #[doc = "8 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_8cycles)
    }
    #[doc = "16 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_16cycles)
    }
    #[doc = "32 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_32cycles)
    }
    #[doc = "64 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_64cycles)
    }
    #[doc = "128 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_128cycles)
    }
    #[doc = "256 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Warmtime::_256cycles)
    }
    #[doc = "512 HFPERCLK cycles"]
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
#[doc = "Field `BIASPROG` reader - VCMP Bias Programming Value"]
pub type BiasprogR = crate::FieldReader;
#[doc = "Field `BIASPROG` writer - VCMP Bias Programming Value"]
pub type BiasprogW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HalfbiasR = crate::BitReader;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HalfbiasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> InactvalR {
        InactvalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HystenR {
        HystenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
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
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BiasprogR {
        BiasprogR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HalfbiasR {
        HalfbiasR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&mut self) -> InactvalW<'_, CtrlSpec> {
        InactvalW::new(self, 2)
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&mut self) -> HystenW<'_, CtrlSpec> {
        HystenW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
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
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BiasprogW<'_, CtrlSpec> {
        BiasprogW::new(self, 24)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HalfbiasW<'_, CtrlSpec> {
        HalfbiasW::new(self, 30)
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
