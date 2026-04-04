#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "HFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxomode {
    #[doc = "0: 4-25 MHz crystal oscillator."]
    Xtal = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    Bufextclk = 1,
    #[doc = "2: Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    Digextclk = 2,
}
impl From<Hfxomode> for u8 {
    #[inline(always)]
    fn from(variant: Hfxomode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxomode {
    type Ux = u8;
}
impl crate::IsEnum for Hfxomode {}
#[doc = "Field `HFXOMODE` reader - HFXO Mode"]
pub type HfxomodeR = crate::FieldReader<Hfxomode>;
impl HfxomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfxomode> {
        match self.bits {
            0 => Some(Hfxomode::Xtal),
            1 => Some(Hfxomode::Bufextclk),
            2 => Some(Hfxomode::Digextclk),
            _ => None,
        }
    }
    #[doc = "4-25 MHz crystal oscillator."]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Hfxomode::Xtal
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == Hfxomode::Bufextclk
    }
    #[doc = "Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == Hfxomode::Digextclk
    }
}
#[doc = "Field `HFXOMODE` writer - HFXO Mode"]
pub type HfxomodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hfxomode>;
impl<'a, REG> HfxomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-25 MHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxomode::Xtal)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-25 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxomode::Bufextclk)
    }
    #[doc = "Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxomode::Digextclk)
    }
}
#[doc = "HFXO Start-up Boost Current\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxoboost {
    #[doc = "0: 50 %."]
    _50pcent = 0,
    #[doc = "1: 70 %."]
    _70pcent = 1,
    #[doc = "2: 80 %."]
    _80pcent = 2,
    #[doc = "3: 100 % (default)."]
    _100pcent = 3,
}
impl From<Hfxoboost> for u8 {
    #[inline(always)]
    fn from(variant: Hfxoboost) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxoboost {
    type Ux = u8;
}
impl crate::IsEnum for Hfxoboost {}
#[doc = "Field `HFXOBOOST` reader - HFXO Start-up Boost Current"]
pub type HfxoboostR = crate::FieldReader<Hfxoboost>;
impl HfxoboostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxoboost {
        match self.bits {
            0 => Hfxoboost::_50pcent,
            1 => Hfxoboost::_70pcent,
            2 => Hfxoboost::_80pcent,
            3 => Hfxoboost::_100pcent,
            _ => unreachable!(),
        }
    }
    #[doc = "50 %."]
    #[inline(always)]
    pub fn is_50pcent(&self) -> bool {
        *self == Hfxoboost::_50pcent
    }
    #[doc = "70 %."]
    #[inline(always)]
    pub fn is_70pcent(&self) -> bool {
        *self == Hfxoboost::_70pcent
    }
    #[doc = "80 %."]
    #[inline(always)]
    pub fn is_80pcent(&self) -> bool {
        *self == Hfxoboost::_80pcent
    }
    #[doc = "100 % (default)."]
    #[inline(always)]
    pub fn is_100pcent(&self) -> bool {
        *self == Hfxoboost::_100pcent
    }
}
#[doc = "Field `HFXOBOOST` writer - HFXO Start-up Boost Current"]
pub type HfxoboostW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hfxoboost, crate::Safe>;
impl<'a, REG> HfxoboostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "50 %."]
    #[inline(always)]
    pub fn _50pcent(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxoboost::_50pcent)
    }
    #[doc = "70 %."]
    #[inline(always)]
    pub fn _70pcent(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxoboost::_70pcent)
    }
    #[doc = "80 %."]
    #[inline(always)]
    pub fn _80pcent(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxoboost::_80pcent)
    }
    #[doc = "100 % (default)."]
    #[inline(always)]
    pub fn _100pcent(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxoboost::_100pcent)
    }
}
#[doc = "Field `HFXOBUFCUR` reader - HFXO Boost Buffer Current"]
pub type HfxobufcurR = crate::FieldReader;
#[doc = "Field `HFXOBUFCUR` writer - HFXO Boost Buffer Current"]
pub type HfxobufcurW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HFXOGLITCHDETEN` reader - HFXO Glitch Detector Enable"]
pub type HfxoglitchdetenR = crate::BitReader;
#[doc = "Field `HFXOGLITCHDETEN` writer - HFXO Glitch Detector Enable"]
pub type HfxoglitchdetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "HFXO Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxotimeout {
    #[doc = "0: Timeout period of 8 cycles."]
    _8cycles = 0,
    #[doc = "1: Timeout period of 256 cycles."]
    _256cycles = 1,
    #[doc = "2: Timeout period of 1024 cycles."]
    _1kcycles = 2,
    #[doc = "3: Timeout period of 16384 cycles."]
    _16kcycles = 3,
}
impl From<Hfxotimeout> for u8 {
    #[inline(always)]
    fn from(variant: Hfxotimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxotimeout {
    type Ux = u8;
}
impl crate::IsEnum for Hfxotimeout {}
#[doc = "Field `HFXOTIMEOUT` reader - HFXO Timeout"]
pub type HfxotimeoutR = crate::FieldReader<Hfxotimeout>;
impl HfxotimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxotimeout {
        match self.bits {
            0 => Hfxotimeout::_8cycles,
            1 => Hfxotimeout::_256cycles,
            2 => Hfxotimeout::_1kcycles,
            3 => Hfxotimeout::_16kcycles,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == Hfxotimeout::_8cycles
    }
    #[doc = "Timeout period of 256 cycles."]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Hfxotimeout::_256cycles
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == Hfxotimeout::_1kcycles
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == Hfxotimeout::_16kcycles
    }
}
#[doc = "Field `HFXOTIMEOUT` writer - HFXO Timeout"]
pub type HfxotimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hfxotimeout, crate::Safe>;
impl<'a, REG> HfxotimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxotimeout::_8cycles)
    }
    #[doc = "Timeout period of 256 cycles."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxotimeout::_256cycles)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxotimeout::_1kcycles)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxotimeout::_16kcycles)
    }
}
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfxomode {
    #[doc = "0: 32.768 kHz crystal oscillator."]
    Xtal = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    Bufextclk = 1,
    #[doc = "2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    Digextclk = 2,
}
impl From<Lfxomode> for u8 {
    #[inline(always)]
    fn from(variant: Lfxomode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfxomode {
    type Ux = u8;
}
impl crate::IsEnum for Lfxomode {}
#[doc = "Field `LFXOMODE` reader - LFXO Mode"]
pub type LfxomodeR = crate::FieldReader<Lfxomode>;
impl LfxomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lfxomode> {
        match self.bits {
            0 => Some(Lfxomode::Xtal),
            1 => Some(Lfxomode::Bufextclk),
            2 => Some(Lfxomode::Digextclk),
            _ => None,
        }
    }
    #[doc = "32.768 kHz crystal oscillator."]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Lfxomode::Xtal
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == Lfxomode::Bufextclk
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == Lfxomode::Digextclk
    }
}
#[doc = "Field `LFXOMODE` writer - LFXO Mode"]
pub type LfxomodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfxomode>;
impl<'a, REG> LfxomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768 kHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxomode::Xtal)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxomode::Bufextclk)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxomode::Digextclk)
    }
}
#[doc = "Field `LFXOBOOST` reader - LFXO Start-up Boost Current"]
pub type LfxoboostR = crate::BitReader;
#[doc = "Field `LFXOBOOST` writer - LFXO Start-up Boost Current"]
pub type LfxoboostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFCLKDIV` reader - HFCLK Division"]
pub type HfclkdivR = crate::FieldReader;
#[doc = "Field `HFCLKDIV` writer - HFCLK Division"]
pub type HfclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LFXOBUFCUR` reader - LFXO Boost Buffer Current"]
pub type LfxobufcurR = crate::BitReader;
#[doc = "Field `LFXOBUFCUR` writer - LFXO Boost Buffer Current"]
pub type LfxobufcurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LFXO Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfxotimeout {
    #[doc = "0: Timeout period of 8 cycles."]
    _8cycles = 0,
    #[doc = "1: Timeout period of 1024 cycles."]
    _1kcycles = 1,
    #[doc = "2: Timeout period of 16384 cycles."]
    _16kcycles = 2,
    #[doc = "3: Timeout period of 32768 cycles."]
    _32kcycles = 3,
}
impl From<Lfxotimeout> for u8 {
    #[inline(always)]
    fn from(variant: Lfxotimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfxotimeout {
    type Ux = u8;
}
impl crate::IsEnum for Lfxotimeout {}
#[doc = "Field `LFXOTIMEOUT` reader - LFXO Timeout"]
pub type LfxotimeoutR = crate::FieldReader<Lfxotimeout>;
impl LfxotimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxotimeout {
        match self.bits {
            0 => Lfxotimeout::_8cycles,
            1 => Lfxotimeout::_1kcycles,
            2 => Lfxotimeout::_16kcycles,
            3 => Lfxotimeout::_32kcycles,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == Lfxotimeout::_8cycles
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == Lfxotimeout::_1kcycles
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == Lfxotimeout::_16kcycles
    }
    #[doc = "Timeout period of 32768 cycles."]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == Lfxotimeout::_32kcycles
    }
}
#[doc = "Field `LFXOTIMEOUT` writer - LFXO Timeout"]
pub type LfxotimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfxotimeout, crate::Safe>;
impl<'a, REG> LfxotimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxotimeout::_8cycles)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxotimeout::_1kcycles)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxotimeout::_16kcycles)
    }
    #[doc = "Timeout period of 32768 cycles."]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxotimeout::_32kcycles)
    }
}
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel0 {
    #[doc = "0: HFRCO (directly from oscillator)."]
    Hfrco = 0,
    #[doc = "1: HFXO (directly from oscillator)."]
    Hfxo = 1,
    #[doc = "2: HFCLK/2."]
    Hfclk2 = 2,
    #[doc = "3: HFCLK/4."]
    Hfclk4 = 3,
    #[doc = "4: HFCLK/8."]
    Hfclk8 = 4,
    #[doc = "5: HFCLK/16."]
    Hfclk16 = 5,
    #[doc = "6: ULFRCO (directly from oscillator)."]
    Ulfrco = 6,
    #[doc = "7: AUXHFRCO (directly from oscillator)."]
    Auxhfrco = 7,
}
impl From<Clkoutsel0> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel0 {}
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type Clkoutsel0R = crate::FieldReader<Clkoutsel0>;
impl Clkoutsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkoutsel0 {
        match self.bits {
            0 => Clkoutsel0::Hfrco,
            1 => Clkoutsel0::Hfxo,
            2 => Clkoutsel0::Hfclk2,
            3 => Clkoutsel0::Hfclk4,
            4 => Clkoutsel0::Hfclk8,
            5 => Clkoutsel0::Hfclk16,
            6 => Clkoutsel0::Ulfrco,
            7 => Clkoutsel0::Auxhfrco,
            _ => unreachable!(),
        }
    }
    #[doc = "HFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Clkoutsel0::Hfrco
    }
    #[doc = "HFXO (directly from oscillator)."]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clkoutsel0::Hfxo
    }
    #[doc = "HFCLK/2."]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == Clkoutsel0::Hfclk2
    }
    #[doc = "HFCLK/4."]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == Clkoutsel0::Hfclk4
    }
    #[doc = "HFCLK/8."]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == Clkoutsel0::Hfclk8
    }
    #[doc = "HFCLK/16."]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == Clkoutsel0::Hfclk16
    }
    #[doc = "ULFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clkoutsel0::Ulfrco
    }
    #[doc = "AUXHFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Clkoutsel0::Auxhfrco
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type Clkoutsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkoutsel0, crate::Safe>;
impl<'a, REG> Clkoutsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfrco)
    }
    #[doc = "HFXO (directly from oscillator)."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfxo)
    }
    #[doc = "HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfclk2)
    }
    #[doc = "HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfclk4)
    }
    #[doc = "HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfclk8)
    }
    #[doc = "HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfclk16)
    }
    #[doc = "ULFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Ulfrco)
    }
    #[doc = "AUXHFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Auxhfrco)
    }
}
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel1 {
    #[doc = "0: LFRCO (directly from oscillator)."]
    Lfrco = 0,
    #[doc = "1: LFXO (directly from oscillator)."]
    Lfxo = 1,
    #[doc = "2: HFCLK (undivided)."]
    Hfclk = 2,
    #[doc = "3: LFXO (qualified)."]
    Lfxoq = 3,
    #[doc = "4: HFXO (qualified)."]
    Hfxoq = 4,
    #[doc = "5: LFRCO (qualified)."]
    Lfrcoq = 5,
    #[doc = "6: HFRCO (qualified)."]
    Hfrcoq = 6,
    #[doc = "7: AUXHFRCO (qualified)."]
    Auxhfrcoq = 7,
    #[doc = "8: USHFRCO"]
    Ushfrco = 8,
}
impl From<Clkoutsel1> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel1 {}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type Clkoutsel1R = crate::FieldReader<Clkoutsel1>;
impl Clkoutsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel1> {
        match self.bits {
            0 => Some(Clkoutsel1::Lfrco),
            1 => Some(Clkoutsel1::Lfxo),
            2 => Some(Clkoutsel1::Hfclk),
            3 => Some(Clkoutsel1::Lfxoq),
            4 => Some(Clkoutsel1::Hfxoq),
            5 => Some(Clkoutsel1::Lfrcoq),
            6 => Some(Clkoutsel1::Hfrcoq),
            7 => Some(Clkoutsel1::Auxhfrcoq),
            8 => Some(Clkoutsel1::Ushfrco),
            _ => None,
        }
    }
    #[doc = "LFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clkoutsel1::Lfrco
    }
    #[doc = "LFXO (directly from oscillator)."]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clkoutsel1::Lfxo
    }
    #[doc = "HFCLK (undivided)."]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Clkoutsel1::Hfclk
    }
    #[doc = "LFXO (qualified)."]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == Clkoutsel1::Lfxoq
    }
    #[doc = "HFXO (qualified)."]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == Clkoutsel1::Hfxoq
    }
    #[doc = "LFRCO (qualified)."]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == Clkoutsel1::Lfrcoq
    }
    #[doc = "HFRCO (qualified)."]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == Clkoutsel1::Hfrcoq
    }
    #[doc = "AUXHFRCO (qualified)."]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == Clkoutsel1::Auxhfrcoq
    }
    #[doc = "USHFRCO"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Clkoutsel1::Ushfrco
    }
}
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type Clkoutsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Clkoutsel1>;
impl<'a, REG> Clkoutsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfrco)
    }
    #[doc = "LFXO (directly from oscillator)."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfxo)
    }
    #[doc = "HFCLK (undivided)."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfclk)
    }
    #[doc = "LFXO (qualified)."]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfxoq)
    }
    #[doc = "HFXO (qualified)."]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfxoq)
    }
    #[doc = "LFRCO (qualified)."]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfrcoq)
    }
    #[doc = "HFRCO (qualified)."]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfrcoq)
    }
    #[doc = "AUXHFRCO (qualified)."]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Auxhfrcoq)
    }
    #[doc = "USHFRCO"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Ushfrco)
    }
}
impl R {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn hfxomode(&self) -> HfxomodeR {
        HfxomodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn hfxoboost(&self) -> HfxoboostR {
        HfxoboostR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn hfxobufcur(&self) -> HfxobufcurR {
        HfxobufcurR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline(always)]
    pub fn hfxoglitchdeten(&self) -> HfxoglitchdetenR {
        HfxoglitchdetenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline(always)]
    pub fn hfxotimeout(&self) -> HfxotimeoutR {
        HfxotimeoutR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline(always)]
    pub fn lfxomode(&self) -> LfxomodeR {
        LfxomodeR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn lfxoboost(&self) -> LfxoboostR {
        LfxoboostR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - HFCLK Division"]
    #[inline(always)]
    pub fn hfclkdiv(&self) -> HfclkdivR {
        HfclkdivR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn lfxobufcur(&self) -> LfxobufcurR {
        LfxobufcurR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline(always)]
    pub fn lfxotimeout(&self) -> LfxotimeoutR {
        LfxotimeoutR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> Clkoutsel0R {
        Clkoutsel0R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:26 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> Clkoutsel1R {
        Clkoutsel1R::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn hfxomode(&mut self) -> HfxomodeW<'_, CtrlSpec> {
        HfxomodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn hfxoboost(&mut self) -> HfxoboostW<'_, CtrlSpec> {
        HfxoboostW::new(self, 2)
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn hfxobufcur(&mut self) -> HfxobufcurW<'_, CtrlSpec> {
        HfxobufcurW::new(self, 5)
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline(always)]
    pub fn hfxoglitchdeten(&mut self) -> HfxoglitchdetenW<'_, CtrlSpec> {
        HfxoglitchdetenW::new(self, 7)
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline(always)]
    pub fn hfxotimeout(&mut self) -> HfxotimeoutW<'_, CtrlSpec> {
        HfxotimeoutW::new(self, 9)
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline(always)]
    pub fn lfxomode(&mut self) -> LfxomodeW<'_, CtrlSpec> {
        LfxomodeW::new(self, 11)
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn lfxoboost(&mut self) -> LfxoboostW<'_, CtrlSpec> {
        LfxoboostW::new(self, 13)
    }
    #[doc = "Bits 14:16 - HFCLK Division"]
    #[inline(always)]
    pub fn hfclkdiv(&mut self) -> HfclkdivW<'_, CtrlSpec> {
        HfclkdivW::new(self, 14)
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn lfxobufcur(&mut self) -> LfxobufcurW<'_, CtrlSpec> {
        LfxobufcurW::new(self, 17)
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline(always)]
    pub fn lfxotimeout(&mut self) -> LfxotimeoutW<'_, CtrlSpec> {
        LfxotimeoutW::new(self, 18)
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&mut self) -> Clkoutsel0W<'_, CtrlSpec> {
        Clkoutsel0W::new(self, 20)
    }
    #[doc = "Bits 23:26 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&mut self) -> Clkoutsel1W<'_, CtrlSpec> {
        Clkoutsel1W::new(self, 23)
    }
}
#[doc = "CMU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x000c_262c"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x000c_262c;
}
