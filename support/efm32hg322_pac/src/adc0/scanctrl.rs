#[doc = "Register `SCANCTRL` reader"]
pub type R = crate::R<SCANCTRL_SPEC>;
#[doc = "Register `SCANCTRL` writer"]
pub type W = crate::W<SCANCTRL_SPEC>;
#[doc = "Field `REP` reader - Scan Sequence Repetitive Mode"]
pub type REP_R = crate::BitReader;
#[doc = "Field `REP` writer - Scan Sequence Repetitive Mode"]
pub type REP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF` reader - Scan Sequence Differential Mode"]
pub type DIFF_R = crate::BitReader;
#[doc = "Field `DIFF` writer - Scan Sequence Differential Mode"]
pub type DIFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ` reader - Scan Sequence Result Adjustment"]
pub type ADJ_R = crate::BitReader;
#[doc = "Field `ADJ` writer - Scan Sequence Result Adjustment"]
pub type ADJ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - Scan Sequence Resolution Select"]
pub type RES_R = crate::FieldReader<RES_A>;
#[doc = "Scan Sequence Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution"]
    _12BIT = 0,
    #[doc = "1: 8-bit resolution"]
    _8BIT = 1,
    #[doc = "2: 6-bit resolution"]
    _6BIT = 2,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    OVS = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES_A {
    type Ux = u8;
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::_12BIT,
            1 => RES_A::_8BIT,
            2 => RES_A::_6BIT,
            3 => RES_A::OVS,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES_A::_12BIT
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES_A::_8BIT
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES_A::_6BIT
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES_A::OVS
    }
}
#[doc = "Field `RES` writer - Scan Sequence Resolution Select"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RES_A>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::_12BIT)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::_8BIT)
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::OVS)
    }
}
#[doc = "Field `INPUTMASK` reader - Scan Sequence Input Mask"]
pub type INPUTMASK_R = crate::FieldReader;
#[doc = "Field `INPUTMASK` writer - Scan Sequence Input Mask"]
pub type INPUTMASK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REF` reader - Scan Sequence Reference Selection"]
pub type REF_R = crate::FieldReader<REF_A>;
#[doc = "Scan Sequence Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_A {
    #[doc = "0: Internal 1.25 V reference"]
    _1V25 = 0,
    #[doc = "1: Internal 2.5 V reference"]
    _2V5 = 1,
    #[doc = "2: VDD"]
    VDD = 2,
    #[doc = "3: Internal differential 5 V reference"]
    _5VDIFF = 3,
    #[doc = "4: Single ended external reference from ADCn_CH6"]
    EXTSINGLE = 4,
    #[doc = "5: Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    _2XEXTDIFF = 5,
    #[doc = "6: Unbuffered 2xVDD"]
    _2XVDD = 6,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REF_A {
    type Ux = u8;
}
impl REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REF_A> {
        match self.bits {
            0 => Some(REF_A::_1V25),
            1 => Some(REF_A::_2V5),
            2 => Some(REF_A::VDD),
            3 => Some(REF_A::_5VDIFF),
            4 => Some(REF_A::EXTSINGLE),
            5 => Some(REF_A::_2XEXTDIFF),
            6 => Some(REF_A::_2XVDD),
            _ => None,
        }
    }
    #[doc = "Internal 1.25 V reference"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REF_A::_1V25
    }
    #[doc = "Internal 2.5 V reference"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REF_A::_2V5
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REF_A::VDD
    }
    #[doc = "Internal differential 5 V reference"]
    #[inline(always)]
    pub fn is_5vdiff(&self) -> bool {
        *self == REF_A::_5VDIFF
    }
    #[doc = "Single ended external reference from ADCn_CH6"]
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == REF_A::EXTSINGLE
    }
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REF_A::_2XEXTDIFF
    }
    #[doc = "Unbuffered 2xVDD"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF_A::_2XVDD
    }
}
#[doc = "Field `REF` writer - Scan Sequence Reference Selection"]
pub type REF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, REF_A>;
impl<'a, REG> REF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 1.25 V reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_1V25)
    }
    #[doc = "Internal 2.5 V reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_2V5)
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::VDD)
    }
    #[doc = "Internal differential 5 V reference"]
    #[inline(always)]
    pub fn _5vdiff(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_5VDIFF)
    }
    #[doc = "Single ended external reference from ADCn_CH6"]
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::EXTSINGLE)
    }
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_2XEXTDIFF)
    }
    #[doc = "Unbuffered 2xVDD"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_2XVDD)
    }
}
#[doc = "Field `AT` reader - Scan Sample Acquisition Time"]
pub type AT_R = crate::FieldReader<AT_A>;
#[doc = "Scan Sample Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AT_A {
    #[doc = "0: 1 ADC_CLK cycle acquisition time for scan samples"]
    _1CYCLE = 0,
    #[doc = "1: 2 ADC_CLK cycles acquisition time for scan samples"]
    _2CYCLES = 1,
    #[doc = "2: 4 ADC_CLK cycles acquisition time for scan samples"]
    _4CYCLES = 2,
    #[doc = "3: 8 ADC_CLK cycles acquisition time for scan samples"]
    _8CYCLES = 3,
    #[doc = "4: 16 ADC_CLK cycles acquisition time for scan samples"]
    _16CYCLES = 4,
    #[doc = "5: 32 ADC_CLK cycles acquisition time for scan samples"]
    _32CYCLES = 5,
    #[doc = "6: 64 ADC_CLK cycles acquisition time for scan samples"]
    _64CYCLES = 6,
    #[doc = "7: 128 ADC_CLK cycles acquisition time for scan samples"]
    _128CYCLES = 7,
    #[doc = "8: 256 ADC_CLK cycles acquisition time for scan samples"]
    _256CYCLES = 8,
}
impl From<AT_A> for u8 {
    #[inline(always)]
    fn from(variant: AT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AT_A {
    type Ux = u8;
}
impl AT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AT_A> {
        match self.bits {
            0 => Some(AT_A::_1CYCLE),
            1 => Some(AT_A::_2CYCLES),
            2 => Some(AT_A::_4CYCLES),
            3 => Some(AT_A::_8CYCLES),
            4 => Some(AT_A::_16CYCLES),
            5 => Some(AT_A::_32CYCLES),
            6 => Some(AT_A::_64CYCLES),
            7 => Some(AT_A::_128CYCLES),
            8 => Some(AT_A::_256CYCLES),
            _ => None,
        }
    }
    #[doc = "1 ADC_CLK cycle acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT_A::_1CYCLE
    }
    #[doc = "2 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT_A::_2CYCLES
    }
    #[doc = "4 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT_A::_4CYCLES
    }
    #[doc = "8 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT_A::_8CYCLES
    }
    #[doc = "16 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT_A::_16CYCLES
    }
    #[doc = "32 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT_A::_32CYCLES
    }
    #[doc = "64 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT_A::_64CYCLES
    }
    #[doc = "128 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT_A::_128CYCLES
    }
    #[doc = "256 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT_A::_256CYCLES
    }
}
#[doc = "Field `AT` writer - Scan Sample Acquisition Time"]
pub type AT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AT_A>;
impl<'a, REG> AT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 ADC_CLK cycle acquisition time for scan samples"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_1CYCLE)
    }
    #[doc = "2 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_2CYCLES)
    }
    #[doc = "4 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_4CYCLES)
    }
    #[doc = "8 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_8CYCLES)
    }
    #[doc = "16 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_16CYCLES)
    }
    #[doc = "32 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_32CYCLES)
    }
    #[doc = "64 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_64CYCLES)
    }
    #[doc = "128 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_128CYCLES)
    }
    #[doc = "256 ADC_CLK cycles acquisition time for scan samples"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT_A::_256CYCLES)
    }
}
#[doc = "Field `PRSEN` reader - Scan Sequence PRS Trigger Enable"]
pub type PRSEN_R = crate::BitReader;
#[doc = "Field `PRSEN` writer - Scan Sequence PRS Trigger Enable"]
pub type PRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSSEL` reader - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "Scan Sequence PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers scan sequence"]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers scan sequence"]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers scan sequence"]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers scan sequence"]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers scan sequence"]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers scan sequence"]
    PRSCH5 = 5,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL_A {
    type Ux = u8;
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
}
#[doc = "Field `PRSSEL` writer - Scan Sequence PRS Trigger Select"]
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PRSSEL_A>;
impl<'a, REG> PRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers scan sequence"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
}
impl R {
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Scan Sequence Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Scan Sequence Input Mask"]
    #[inline(always)]
    pub fn inputmask(&self) -> INPUTMASK_R {
        INPUTMASK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Scan Sequence Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Scan Sample Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Scan Sequence PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<SCANCTRL_SPEC> {
        REP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<SCANCTRL_SPEC> {
        DIFF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> ADJ_W<SCANCTRL_SPEC> {
        ADJ_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Scan Sequence Resolution Select"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<SCANCTRL_SPEC> {
        RES_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Scan Sequence Input Mask"]
    #[inline(always)]
    #[must_use]
    pub fn inputmask(&mut self) -> INPUTMASK_W<SCANCTRL_SPEC> {
        INPUTMASK_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Scan Sequence Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<SCANCTRL_SPEC> {
        REF_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Scan Sample Acquisition Time"]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AT_W<SCANCTRL_SPEC> {
        AT_W::new(self, 20)
    }
    #[doc = "Bit 24 - Scan Sequence PRS Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsen(&mut self) -> PRSEN_W<SCANCTRL_SPEC> {
        PRSEN_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Scan Sequence PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<SCANCTRL_SPEC> {
        PRSSEL_W::new(self, 28)
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
#[doc = "Scan Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANCTRL_SPEC;
impl crate::RegisterSpec for SCANCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanctrl::R`](R) reader structure"]
impl crate::Readable for SCANCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scanctrl::W`](W) writer structure"]
impl crate::Writable for SCANCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANCTRL to value 0"]
impl crate::Resettable for SCANCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
