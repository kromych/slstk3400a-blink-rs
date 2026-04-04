#[doc = "Register `SINGLECTRL` reader"]
pub type R = crate::R<SinglectrlSpec>;
#[doc = "Register `SINGLECTRL` writer"]
pub type W = crate::W<SinglectrlSpec>;
#[doc = "Field `REP` reader - Single Sample Repetitive Mode"]
pub type RepR = crate::BitReader;
#[doc = "Field `REP` writer - Single Sample Repetitive Mode"]
pub type RepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF` reader - Single Sample Differential Mode"]
pub type DiffR = crate::BitReader;
#[doc = "Field `DIFF` writer - Single Sample Differential Mode"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ` reader - Single Sample Result Adjustment"]
pub type AdjR = crate::BitReader;
#[doc = "Field `ADJ` writer - Single Sample Result Adjustment"]
pub type AdjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Single Sample Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Res {
    #[doc = "0: 12-bit resolution"]
    _12bit = 0,
    #[doc = "1: 8-bit resolution"]
    _8bit = 1,
    #[doc = "2: 6-bit resolution"]
    _6bit = 2,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    Ovs = 3,
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(variant: Res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Res {
    type Ux = u8;
}
impl crate::IsEnum for Res {}
#[doc = "Field `RES` reader - Single Sample Resolution Select"]
pub type ResR = crate::FieldReader<Res>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Res {
        match self.bits {
            0 => Res::_12bit,
            1 => Res::_8bit,
            2 => Res::_6bit,
            3 => Res::Ovs,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Res::_12bit
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Res::_8bit
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Res::_6bit
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == Res::Ovs
    }
}
#[doc = "Field `RES` writer - Single Sample Resolution Select"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, Res, crate::Safe>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Res::_12bit)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Res::_8bit)
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Res::_6bit)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Ovs)
    }
}
#[doc = "Field `INPUTSEL` reader - Single Sample Input Selection"]
pub type InputselR = crate::FieldReader;
#[doc = "Field `INPUTSEL` writer - Single Sample Input Selection"]
pub type InputselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Single Sample Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ref {
    #[doc = "0: Internal 1.25 V reference"]
    _1v25 = 0,
    #[doc = "1: Internal 2.5 V reference"]
    _2v5 = 1,
    #[doc = "2: Buffered VDD"]
    Vdd = 2,
    #[doc = "3: Internal differential 5 V reference"]
    _5vdiff = 3,
    #[doc = "4: Single ended external reference from ADCn_CH6"]
    Extsingle = 4,
    #[doc = "5: Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    _2xextdiff = 5,
    #[doc = "6: Unbuffered 2xVDD"]
    _2xvdd = 6,
}
impl From<Ref> for u8 {
    #[inline(always)]
    fn from(variant: Ref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ref {
    type Ux = u8;
}
impl crate::IsEnum for Ref {}
#[doc = "Field `REF` reader - Single Sample Reference Selection"]
pub type RefR = crate::FieldReader<Ref>;
impl RefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ref> {
        match self.bits {
            0 => Some(Ref::_1v25),
            1 => Some(Ref::_2v5),
            2 => Some(Ref::Vdd),
            3 => Some(Ref::_5vdiff),
            4 => Some(Ref::Extsingle),
            5 => Some(Ref::_2xextdiff),
            6 => Some(Ref::_2xvdd),
            _ => None,
        }
    }
    #[doc = "Internal 1.25 V reference"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == Ref::_1v25
    }
    #[doc = "Internal 2.5 V reference"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == Ref::_2v5
    }
    #[doc = "Buffered VDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Ref::Vdd
    }
    #[doc = "Internal differential 5 V reference"]
    #[inline(always)]
    pub fn is_5vdiff(&self) -> bool {
        *self == Ref::_5vdiff
    }
    #[doc = "Single ended external reference from ADCn_CH6"]
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == Ref::Extsingle
    }
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == Ref::_2xextdiff
    }
    #[doc = "Unbuffered 2xVDD"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == Ref::_2xvdd
    }
}
#[doc = "Field `REF` writer - Single Sample Reference Selection"]
pub type RefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ref>;
impl<'a, REG> RefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 1.25 V reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_1v25)
    }
    #[doc = "Internal 2.5 V reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_2v5)
    }
    #[doc = "Buffered VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::Vdd)
    }
    #[doc = "Internal differential 5 V reference"]
    #[inline(always)]
    pub fn _5vdiff(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_5vdiff)
    }
    #[doc = "Single ended external reference from ADCn_CH6"]
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::Extsingle)
    }
    #[doc = "Differential external reference, 2x(ADCn_CH6 - ADCn_CH7)"]
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_2xextdiff)
    }
    #[doc = "Unbuffered 2xVDD"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut crate::W<REG> {
        self.variant(Ref::_2xvdd)
    }
}
#[doc = "Single Sample Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum At {
    #[doc = "0: 1 ADC_CLK cycle acquisition time for single sample"]
    _1cycle = 0,
    #[doc = "1: 2 ADC_CLK cycles acquisition time for single sample"]
    _2cycles = 1,
    #[doc = "2: 4 ADC_CLK cycles acquisition time for single sample"]
    _4cycles = 2,
    #[doc = "3: 8 ADC_CLK cycles acquisition time for single sample"]
    _8cycles = 3,
    #[doc = "4: 16 ADC_CLK cycles acquisition time for single sample"]
    _16cycles = 4,
    #[doc = "5: 32 ADC_CLK cycles acquisition time for single sample"]
    _32cycles = 5,
    #[doc = "6: 64 ADC_CLK cycles acquisition time for single sample"]
    _64cycles = 6,
    #[doc = "7: 128 ADC_CLK cycles acquisition time for single sample"]
    _128cycles = 7,
    #[doc = "8: 256 ADC_CLK cycles acquisition time for single sample"]
    _256cycles = 8,
}
impl From<At> for u8 {
    #[inline(always)]
    fn from(variant: At) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for At {
    type Ux = u8;
}
impl crate::IsEnum for At {}
#[doc = "Field `AT` reader - Single Sample Acquisition Time"]
pub type AtR = crate::FieldReader<At>;
impl AtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<At> {
        match self.bits {
            0 => Some(At::_1cycle),
            1 => Some(At::_2cycles),
            2 => Some(At::_4cycles),
            3 => Some(At::_8cycles),
            4 => Some(At::_16cycles),
            5 => Some(At::_32cycles),
            6 => Some(At::_64cycles),
            7 => Some(At::_128cycles),
            8 => Some(At::_256cycles),
            _ => None,
        }
    }
    #[doc = "1 ADC_CLK cycle acquisition time for single sample"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == At::_1cycle
    }
    #[doc = "2 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == At::_2cycles
    }
    #[doc = "4 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == At::_4cycles
    }
    #[doc = "8 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == At::_8cycles
    }
    #[doc = "16 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == At::_16cycles
    }
    #[doc = "32 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == At::_32cycles
    }
    #[doc = "64 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == At::_64cycles
    }
    #[doc = "128 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == At::_128cycles
    }
    #[doc = "256 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == At::_256cycles
    }
}
#[doc = "Field `AT` writer - Single Sample Acquisition Time"]
pub type AtW<'a, REG> = crate::FieldWriter<'a, REG, 4, At>;
impl<'a, REG> AtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 ADC_CLK cycle acquisition time for single sample"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut crate::W<REG> {
        self.variant(At::_1cycle)
    }
    #[doc = "2 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_2cycles)
    }
    #[doc = "4 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_4cycles)
    }
    #[doc = "8 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_8cycles)
    }
    #[doc = "16 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_16cycles)
    }
    #[doc = "32 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_32cycles)
    }
    #[doc = "64 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_64cycles)
    }
    #[doc = "128 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_128cycles)
    }
    #[doc = "256 ADC_CLK cycles acquisition time for single sample"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(At::_256cycles)
    }
}
#[doc = "Field `PRSEN` reader - Single Sample PRS Trigger Enable"]
pub type PrsenR = crate::BitReader;
#[doc = "Field `PRSEN` writer - Single Sample PRS Trigger Enable"]
pub type PrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Single Sample PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS ch 0 triggers single sample"]
    Prsch0 = 0,
    #[doc = "1: PRS ch 1 triggers single sample"]
    Prsch1 = 1,
    #[doc = "2: PRS ch 2 triggers single sample"]
    Prsch2 = 2,
    #[doc = "3: PRS ch 3 triggers single sample"]
    Prsch3 = 3,
    #[doc = "4: PRS ch 4 triggers single sample"]
    Prsch4 = 4,
    #[doc = "5: PRS ch 5 triggers single sample"]
    Prsch5 = 5,
}
impl From<Prssel> for u8 {
    #[inline(always)]
    fn from(variant: Prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel {
    type Ux = u8;
}
impl crate::IsEnum for Prssel {}
#[doc = "Field `PRSSEL` reader - Single Sample PRS Trigger Select"]
pub type PrsselR = crate::FieldReader<Prssel>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel> {
        match self.bits {
            0 => Some(Prssel::Prsch0),
            1 => Some(Prssel::Prsch1),
            2 => Some(Prssel::Prsch2),
            3 => Some(Prssel::Prsch3),
            4 => Some(Prssel::Prsch4),
            5 => Some(Prssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 triggers single sample"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS ch 1 triggers single sample"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS ch 2 triggers single sample"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS ch 3 triggers single sample"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS ch 4 triggers single sample"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS ch 5 triggers single sample"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
}
#[doc = "Field `PRSSEL` writer - Single Sample PRS Trigger Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers single sample"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS ch 1 triggers single sample"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS ch 2 triggers single sample"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS ch 3 triggers single sample"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS ch 4 triggers single sample"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS ch 5 triggers single sample"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
}
impl R {
    #[doc = "Bit 0 - Single Sample Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single Sample Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Sample Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> AdjR {
        AdjR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Single Sample Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Single Sample Input Selection"]
    #[inline(always)]
    pub fn inputsel(&self) -> InputselR {
        InputselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Single Sample Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> RefR {
        RefR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Single Sample Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AtR {
        AtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Single Sample PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PrsenR {
        PrsenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Single Sample PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Single Sample Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&mut self) -> RepW<'_, SinglectrlSpec> {
        RepW::new(self, 0)
    }
    #[doc = "Bit 1 - Single Sample Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<'_, SinglectrlSpec> {
        DiffW::new(self, 1)
    }
    #[doc = "Bit 2 - Single Sample Result Adjustment"]
    #[inline(always)]
    pub fn adj(&mut self) -> AdjW<'_, SinglectrlSpec> {
        AdjW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Single Sample Resolution Select"]
    #[inline(always)]
    pub fn res(&mut self) -> ResW<'_, SinglectrlSpec> {
        ResW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Single Sample Input Selection"]
    #[inline(always)]
    pub fn inputsel(&mut self) -> InputselW<'_, SinglectrlSpec> {
        InputselW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Single Sample Reference Selection"]
    #[inline(always)]
    pub fn ref_(&mut self) -> RefW<'_, SinglectrlSpec> {
        RefW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Single Sample Acquisition Time"]
    #[inline(always)]
    pub fn at(&mut self) -> AtW<'_, SinglectrlSpec> {
        AtW::new(self, 20)
    }
    #[doc = "Bit 24 - Single Sample PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&mut self) -> PrsenW<'_, SinglectrlSpec> {
        PrsenW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Single Sample PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, SinglectrlSpec> {
        PrsselW::new(self, 28)
    }
}
#[doc = "Single Sample Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singlectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglectrlSpec;
impl crate::RegisterSpec for SinglectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlectrl::R`](R) reader structure"]
impl crate::Readable for SinglectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`singlectrl::W`](W) writer structure"]
impl crate::Writable for SinglectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGLECTRL to value 0"]
impl crate::Resettable for SinglectrlSpec {}
