#[doc = "Register `INPUTSEL` reader"]
pub type R = crate::R<InputselSpec>;
#[doc = "Register `INPUTSEL` writer"]
pub type W = crate::W<InputselSpec>;
#[doc = "Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Possel {
    #[doc = "0: Channel 0 as positive input."]
    Ch0 = 0,
    #[doc = "1: Channel 1 as positive input."]
    Ch1 = 1,
    #[doc = "2: Channel 2 as positive input."]
    Ch2 = 2,
    #[doc = "3: Channel 3 as positive input."]
    Ch3 = 3,
    #[doc = "4: Channel 4 as positive input."]
    Ch4 = 4,
    #[doc = "5: Channel 5 as positive input."]
    Ch5 = 5,
    #[doc = "6: Channel 6 as positive input."]
    Ch6 = 6,
    #[doc = "7: Channel 7 as positive input."]
    Ch7 = 7,
}
impl From<Possel> for u8 {
    #[inline(always)]
    fn from(variant: Possel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Possel {
    type Ux = u8;
}
impl crate::IsEnum for Possel {}
#[doc = "Field `POSSEL` reader - Positive Input Select"]
pub type PosselR = crate::FieldReader<Possel>;
impl PosselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Possel {
        match self.bits {
            0 => Possel::Ch0,
            1 => Possel::Ch1,
            2 => Possel::Ch2,
            3 => Possel::Ch3,
            4 => Possel::Ch4,
            5 => Possel::Ch5,
            6 => Possel::Ch6,
            7 => Possel::Ch7,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 as positive input."]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == Possel::Ch0
    }
    #[doc = "Channel 1 as positive input."]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == Possel::Ch1
    }
    #[doc = "Channel 2 as positive input."]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == Possel::Ch2
    }
    #[doc = "Channel 3 as positive input."]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == Possel::Ch3
    }
    #[doc = "Channel 4 as positive input."]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == Possel::Ch4
    }
    #[doc = "Channel 5 as positive input."]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == Possel::Ch5
    }
    #[doc = "Channel 6 as positive input."]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == Possel::Ch6
    }
    #[doc = "Channel 7 as positive input."]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == Possel::Ch7
    }
}
#[doc = "Field `POSSEL` writer - Positive Input Select"]
pub type PosselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Possel, crate::Safe>;
impl<'a, REG> PosselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 as positive input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch0)
    }
    #[doc = "Channel 1 as positive input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch1)
    }
    #[doc = "Channel 2 as positive input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch2)
    }
    #[doc = "Channel 3 as positive input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch3)
    }
    #[doc = "Channel 4 as positive input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch4)
    }
    #[doc = "Channel 5 as positive input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch5)
    }
    #[doc = "Channel 6 as positive input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch6)
    }
    #[doc = "Channel 7 as positive input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Ch7)
    }
}
#[doc = "Negative Input Select\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Negsel {
    #[doc = "0: Channel 0 as negative input."]
    Ch0 = 0,
    #[doc = "1: Channel 1 as negative input."]
    Ch1 = 1,
    #[doc = "2: Channel 2 as negative input."]
    Ch2 = 2,
    #[doc = "3: Channel 3 as negative input."]
    Ch3 = 3,
    #[doc = "4: Channel 4 as negative input."]
    Ch4 = 4,
    #[doc = "5: Channel 5 as negative input."]
    Ch5 = 5,
    #[doc = "6: Channel 6 as negative input."]
    Ch6 = 6,
    #[doc = "7: Channel 7 as negative input."]
    Ch7 = 7,
    #[doc = "8: 1.25 V as negative input."]
    _1v25 = 8,
    #[doc = "9: 2.5 V as negative input."]
    _2v5 = 9,
    #[doc = "10: Scaled VDD as negative input."]
    Vdd = 10,
    #[doc = "11: Capacitive sense mode."]
    Capsense = 11,
}
impl From<Negsel> for u8 {
    #[inline(always)]
    fn from(variant: Negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Negsel {
    type Ux = u8;
}
impl crate::IsEnum for Negsel {}
#[doc = "Field `NEGSEL` reader - Negative Input Select"]
pub type NegselR = crate::FieldReader<Negsel>;
impl NegselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Negsel> {
        match self.bits {
            0 => Some(Negsel::Ch0),
            1 => Some(Negsel::Ch1),
            2 => Some(Negsel::Ch2),
            3 => Some(Negsel::Ch3),
            4 => Some(Negsel::Ch4),
            5 => Some(Negsel::Ch5),
            6 => Some(Negsel::Ch6),
            7 => Some(Negsel::Ch7),
            8 => Some(Negsel::_1v25),
            9 => Some(Negsel::_2v5),
            10 => Some(Negsel::Vdd),
            11 => Some(Negsel::Capsense),
            _ => None,
        }
    }
    #[doc = "Channel 0 as negative input."]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == Negsel::Ch0
    }
    #[doc = "Channel 1 as negative input."]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == Negsel::Ch1
    }
    #[doc = "Channel 2 as negative input."]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == Negsel::Ch2
    }
    #[doc = "Channel 3 as negative input."]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == Negsel::Ch3
    }
    #[doc = "Channel 4 as negative input."]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == Negsel::Ch4
    }
    #[doc = "Channel 5 as negative input."]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == Negsel::Ch5
    }
    #[doc = "Channel 6 as negative input."]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == Negsel::Ch6
    }
    #[doc = "Channel 7 as negative input."]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == Negsel::Ch7
    }
    #[doc = "1.25 V as negative input."]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == Negsel::_1v25
    }
    #[doc = "2.5 V as negative input."]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == Negsel::_2v5
    }
    #[doc = "Scaled VDD as negative input."]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Negsel::Vdd
    }
    #[doc = "Capacitive sense mode."]
    #[inline(always)]
    pub fn is_capsense(&self) -> bool {
        *self == Negsel::Capsense
    }
}
#[doc = "Field `NEGSEL` writer - Negative Input Select"]
pub type NegselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Negsel>;
impl<'a, REG> NegselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 as negative input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch0)
    }
    #[doc = "Channel 1 as negative input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch1)
    }
    #[doc = "Channel 2 as negative input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch2)
    }
    #[doc = "Channel 3 as negative input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch3)
    }
    #[doc = "Channel 4 as negative input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch4)
    }
    #[doc = "Channel 5 as negative input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch5)
    }
    #[doc = "Channel 6 as negative input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch6)
    }
    #[doc = "Channel 7 as negative input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Ch7)
    }
    #[doc = "1.25 V as negative input."]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::_1v25)
    }
    #[doc = "2.5 V as negative input."]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::_2v5)
    }
    #[doc = "Scaled VDD as negative input."]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vdd)
    }
    #[doc = "Capacitive sense mode."]
    #[inline(always)]
    pub fn capsense(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Capsense)
    }
}
#[doc = "Field `VDDLEVEL` reader - VDD Reference Level"]
pub type VddlevelR = crate::FieldReader;
#[doc = "Field `VDDLEVEL` writer - VDD Reference Level"]
pub type VddlevelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LPREF` reader - Low Power Reference Mode"]
pub type LprefR = crate::BitReader;
#[doc = "Field `LPREF` writer - Low Power Reference Mode"]
pub type LprefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRESEN` reader - Capacitive Sense Mode Internal Resistor Enable"]
pub type CsresenR = crate::BitReader;
#[doc = "Field `CSRESEN` writer - Capacitive Sense Mode Internal Resistor Enable"]
pub type CsresenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capacitive Sense Mode Internal Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csressel {
    #[doc = "0: Internal capacitive sense resistor value 0."]
    Res0 = 0,
    #[doc = "1: Internal capacitive sense resistor value 1."]
    Res1 = 1,
    #[doc = "2: Internal capacitive sense resistor value 2."]
    Res2 = 2,
    #[doc = "3: Internal capacitive sense resistor value 3."]
    Res3 = 3,
}
impl From<Csressel> for u8 {
    #[inline(always)]
    fn from(variant: Csressel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csressel {
    type Ux = u8;
}
impl crate::IsEnum for Csressel {}
#[doc = "Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor Select"]
pub type CsresselR = crate::FieldReader<Csressel>;
impl CsresselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csressel {
        match self.bits {
            0 => Csressel::Res0,
            1 => Csressel::Res1,
            2 => Csressel::Res2,
            3 => Csressel::Res3,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal capacitive sense resistor value 0."]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == Csressel::Res0
    }
    #[doc = "Internal capacitive sense resistor value 1."]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == Csressel::Res1
    }
    #[doc = "Internal capacitive sense resistor value 2."]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == Csressel::Res2
    }
    #[doc = "Internal capacitive sense resistor value 3."]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == Csressel::Res3
    }
}
#[doc = "Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor Select"]
pub type CsresselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csressel, crate::Safe>;
impl<'a, REG> CsresselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal capacitive sense resistor value 0."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res0)
    }
    #[doc = "Internal capacitive sense resistor value 1."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res1)
    }
    #[doc = "Internal capacitive sense resistor value 2."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res2)
    }
    #[doc = "Internal capacitive sense resistor value 3."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> PosselR {
        PosselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NegselR {
        NegselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    pub fn vddlevel(&self) -> VddlevelR {
        VddlevelR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    pub fn lpref(&self) -> LprefR {
        LprefR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&self) -> CsresenR {
        CsresenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&self) -> CsresselR {
        CsresselR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&mut self) -> PosselW<'_, InputselSpec> {
        PosselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NegselW<'_, InputselSpec> {
        NegselW::new(self, 4)
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    pub fn vddlevel(&mut self) -> VddlevelW<'_, InputselSpec> {
        VddlevelW::new(self, 8)
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    pub fn lpref(&mut self) -> LprefW<'_, InputselSpec> {
        LprefW::new(self, 16)
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&mut self) -> CsresenW<'_, InputselSpec> {
        CsresenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&mut self) -> CsresselW<'_, InputselSpec> {
        CsresselW::new(self, 28)
    }
}
#[doc = "Input Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inputsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputselSpec;
impl crate::RegisterSpec for InputselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputsel::R`](R) reader structure"]
impl crate::Readable for InputselSpec {}
#[doc = "`write(|w| ..)` method takes [`inputsel::W`](W) writer structure"]
impl crate::Writable for InputselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTSEL to value 0x0001_0080"]
impl crate::Resettable for InputselSpec {
    const RESET_VALUE: u32 = 0x0001_0080;
}
