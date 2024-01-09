#[doc = "Register `INPUTSEL` reader"]
pub type R = crate::R<INPUTSEL_SPEC>;
#[doc = "Register `INPUTSEL` writer"]
pub type W = crate::W<INPUTSEL_SPEC>;
#[doc = "Field `POSSEL` reader - Positive Input Select"]
pub type POSSEL_R = crate::FieldReader<POSSEL_A>;
#[doc = "Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POSSEL_A {
    #[doc = "0: Channel 0 as positive input."]
    CH0 = 0,
    #[doc = "1: Channel 1 as positive input."]
    CH1 = 1,
    #[doc = "2: Channel 2 as positive input."]
    CH2 = 2,
    #[doc = "3: Channel 3 as positive input."]
    CH3 = 3,
    #[doc = "4: Channel 4 as positive input."]
    CH4 = 4,
    #[doc = "5: Channel 5 as positive input."]
    CH5 = 5,
    #[doc = "6: Channel 6 as positive input."]
    CH6 = 6,
    #[doc = "7: Channel 7 as positive input."]
    CH7 = 7,
}
impl From<POSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POSSEL_A {
    type Ux = u8;
}
impl POSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POSSEL_A {
        match self.bits {
            0 => POSSEL_A::CH0,
            1 => POSSEL_A::CH1,
            2 => POSSEL_A::CH2,
            3 => POSSEL_A::CH3,
            4 => POSSEL_A::CH4,
            5 => POSSEL_A::CH5,
            6 => POSSEL_A::CH6,
            7 => POSSEL_A::CH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 as positive input."]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == POSSEL_A::CH0
    }
    #[doc = "Channel 1 as positive input."]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == POSSEL_A::CH1
    }
    #[doc = "Channel 2 as positive input."]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == POSSEL_A::CH2
    }
    #[doc = "Channel 3 as positive input."]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == POSSEL_A::CH3
    }
    #[doc = "Channel 4 as positive input."]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == POSSEL_A::CH4
    }
    #[doc = "Channel 5 as positive input."]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == POSSEL_A::CH5
    }
    #[doc = "Channel 6 as positive input."]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == POSSEL_A::CH6
    }
    #[doc = "Channel 7 as positive input."]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == POSSEL_A::CH7
    }
}
#[doc = "Field `POSSEL` writer - Positive Input Select"]
pub type POSSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, POSSEL_A>;
impl<'a, REG> POSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 as positive input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH0)
    }
    #[doc = "Channel 1 as positive input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH1)
    }
    #[doc = "Channel 2 as positive input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH2)
    }
    #[doc = "Channel 3 as positive input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH3)
    }
    #[doc = "Channel 4 as positive input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH4)
    }
    #[doc = "Channel 5 as positive input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH5)
    }
    #[doc = "Channel 6 as positive input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH6)
    }
    #[doc = "Channel 7 as positive input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut crate::W<REG> {
        self.variant(POSSEL_A::CH7)
    }
}
#[doc = "Field `NEGSEL` reader - Negative Input Select"]
pub type NEGSEL_R = crate::FieldReader<NEGSEL_A>;
#[doc = "Negative Input Select\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NEGSEL_A {
    #[doc = "0: Channel 0 as negative input."]
    CH0 = 0,
    #[doc = "1: Channel 1 as negative input."]
    CH1 = 1,
    #[doc = "2: Channel 2 as negative input."]
    CH2 = 2,
    #[doc = "3: Channel 3 as negative input."]
    CH3 = 3,
    #[doc = "4: Channel 4 as negative input."]
    CH4 = 4,
    #[doc = "5: Channel 5 as negative input."]
    CH5 = 5,
    #[doc = "6: Channel 6 as negative input."]
    CH6 = 6,
    #[doc = "7: Channel 7 as negative input."]
    CH7 = 7,
    #[doc = "8: 1.25 V as negative input."]
    _1V25 = 8,
    #[doc = "9: 2.5 V as negative input."]
    _2V5 = 9,
    #[doc = "10: Scaled VDD as negative input."]
    VDD = 10,
    #[doc = "11: Capacitive sense mode."]
    CAPSENSE = 11,
}
impl From<NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEGSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NEGSEL_A {
    type Ux = u8;
}
impl NEGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NEGSEL_A> {
        match self.bits {
            0 => Some(NEGSEL_A::CH0),
            1 => Some(NEGSEL_A::CH1),
            2 => Some(NEGSEL_A::CH2),
            3 => Some(NEGSEL_A::CH3),
            4 => Some(NEGSEL_A::CH4),
            5 => Some(NEGSEL_A::CH5),
            6 => Some(NEGSEL_A::CH6),
            7 => Some(NEGSEL_A::CH7),
            8 => Some(NEGSEL_A::_1V25),
            9 => Some(NEGSEL_A::_2V5),
            10 => Some(NEGSEL_A::VDD),
            11 => Some(NEGSEL_A::CAPSENSE),
            _ => None,
        }
    }
    #[doc = "Channel 0 as negative input."]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == NEGSEL_A::CH0
    }
    #[doc = "Channel 1 as negative input."]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == NEGSEL_A::CH1
    }
    #[doc = "Channel 2 as negative input."]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == NEGSEL_A::CH2
    }
    #[doc = "Channel 3 as negative input."]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == NEGSEL_A::CH3
    }
    #[doc = "Channel 4 as negative input."]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == NEGSEL_A::CH4
    }
    #[doc = "Channel 5 as negative input."]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == NEGSEL_A::CH5
    }
    #[doc = "Channel 6 as negative input."]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == NEGSEL_A::CH6
    }
    #[doc = "Channel 7 as negative input."]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == NEGSEL_A::CH7
    }
    #[doc = "1.25 V as negative input."]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == NEGSEL_A::_1V25
    }
    #[doc = "2.5 V as negative input."]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == NEGSEL_A::_2V5
    }
    #[doc = "Scaled VDD as negative input."]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == NEGSEL_A::VDD
    }
    #[doc = "Capacitive sense mode."]
    #[inline(always)]
    pub fn is_capsense(&self) -> bool {
        *self == NEGSEL_A::CAPSENSE
    }
}
#[doc = "Field `NEGSEL` writer - Negative Input Select"]
pub type NEGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, NEGSEL_A>;
impl<'a, REG> NEGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 as negative input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH0)
    }
    #[doc = "Channel 1 as negative input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH1)
    }
    #[doc = "Channel 2 as negative input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH2)
    }
    #[doc = "Channel 3 as negative input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH3)
    }
    #[doc = "Channel 4 as negative input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH4)
    }
    #[doc = "Channel 5 as negative input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH5)
    }
    #[doc = "Channel 6 as negative input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH6)
    }
    #[doc = "Channel 7 as negative input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CH7)
    }
    #[doc = "1.25 V as negative input."]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::_1V25)
    }
    #[doc = "2.5 V as negative input."]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::_2V5)
    }
    #[doc = "Scaled VDD as negative input."]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::VDD)
    }
    #[doc = "Capacitive sense mode."]
    #[inline(always)]
    pub fn capsense(self) -> &'a mut crate::W<REG> {
        self.variant(NEGSEL_A::CAPSENSE)
    }
}
#[doc = "Field `VDDLEVEL` reader - VDD Reference Level"]
pub type VDDLEVEL_R = crate::FieldReader;
#[doc = "Field `VDDLEVEL` writer - VDD Reference Level"]
pub type VDDLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LPREF` reader - Low Power Reference Mode"]
pub type LPREF_R = crate::BitReader;
#[doc = "Field `LPREF` writer - Low Power Reference Mode"]
pub type LPREF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRESEN` reader - Capacitive Sense Mode Internal Resistor Enable"]
pub type CSRESEN_R = crate::BitReader;
#[doc = "Field `CSRESEN` writer - Capacitive Sense Mode Internal Resistor Enable"]
pub type CSRESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor Select"]
pub type CSRESSEL_R = crate::FieldReader<CSRESSEL_A>;
#[doc = "Capacitive Sense Mode Internal Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSRESSEL_A {
    #[doc = "0: Internal capacitive sense resistor value 0."]
    RES0 = 0,
    #[doc = "1: Internal capacitive sense resistor value 1."]
    RES1 = 1,
    #[doc = "2: Internal capacitive sense resistor value 2."]
    RES2 = 2,
    #[doc = "3: Internal capacitive sense resistor value 3."]
    RES3 = 3,
}
impl From<CSRESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRESSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSRESSEL_A {
    type Ux = u8;
}
impl CSRESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSRESSEL_A {
        match self.bits {
            0 => CSRESSEL_A::RES0,
            1 => CSRESSEL_A::RES1,
            2 => CSRESSEL_A::RES2,
            3 => CSRESSEL_A::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal capacitive sense resistor value 0."]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSEL_A::RES0
    }
    #[doc = "Internal capacitive sense resistor value 1."]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSEL_A::RES1
    }
    #[doc = "Internal capacitive sense resistor value 2."]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSEL_A::RES2
    }
    #[doc = "Internal capacitive sense resistor value 3."]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSEL_A::RES3
    }
}
#[doc = "Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor Select"]
pub type CSRESSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CSRESSEL_A>;
impl<'a, REG> CSRESSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal capacitive sense resistor value 0."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    pub fn vddlevel(&self) -> VDDLEVEL_R {
        VDDLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    pub fn lpref(&self) -> LPREF_R {
        LPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&self) -> CSRESEN_R {
        CSRESEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&self) -> CSRESSEL_R {
        CSRESSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn possel(&mut self) -> POSSEL_W<INPUTSEL_SPEC> {
        POSSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn negsel(&mut self) -> NEGSEL_W<INPUTSEL_SPEC> {
        NEGSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    #[must_use]
    pub fn vddlevel(&mut self) -> VDDLEVEL_W<INPUTSEL_SPEC> {
        VDDLEVEL_W::new(self, 8)
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpref(&mut self) -> LPREF_W<INPUTSEL_SPEC> {
        LPREF_W::new(self, 16)
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csresen(&mut self) -> CSRESEN_W<INPUTSEL_SPEC> {
        CSRESEN_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    #[must_use]
    pub fn csressel(&mut self) -> CSRESSEL_W<INPUTSEL_SPEC> {
        CSRESSEL_W::new(self, 28)
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
#[doc = "Input Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTSEL_SPEC;
impl crate::RegisterSpec for INPUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputsel::R`](R) reader structure"]
impl crate::Readable for INPUTSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputsel::W`](W) writer structure"]
impl crate::Writable for INPUTSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPUTSEL to value 0x0001_0080"]
impl crate::Resettable for INPUTSEL_SPEC {
    const RESET_VALUE: u32 = 0x0001_0080;
}
