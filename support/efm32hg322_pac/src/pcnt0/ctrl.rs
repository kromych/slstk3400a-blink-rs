#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: The module is disabled."]
    Disable = 0,
    #[doc = "1: Single input LFACLK oversampling mode (available in EM0-EM2)."]
    Ovssingle = 1,
    #[doc = "2: Externally clocked single input counter mode (available in EM0-EM3)."]
    Extclksingle = 2,
    #[doc = "3: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    Extclkquad = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Mode Select"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Disable,
            1 => Mode::Ovssingle,
            2 => Mode::Extclksingle,
            3 => Mode::Extclkquad,
            _ => unreachable!(),
        }
    }
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mode::Disable
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == Mode::Ovssingle
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == Mode::Extclksingle
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == Mode::Extclkquad
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Disable)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovssingle)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclksingle)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclkquad)
    }
}
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Control"]
pub type CntdirR = crate::BitReader;
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Control"]
pub type CntdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EdgeR = crate::BitReader;
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILT` reader - Enable Digital Pulse Width Filter"]
pub type FiltR = crate::BitReader;
#[doc = "Field `FILT` writer - Enable Digital Pulse Width Filter"]
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTEN` reader - Enable PCNT Clock Domain Reset"]
pub type RstenR = crate::BitReader;
#[doc = "Field `RSTEN` writer - Enable PCNT Clock Domain Reset"]
pub type RstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXCNTRSTEN` reader - Enable AUXCNT Reset"]
pub type AuxcntrstenR = crate::BitReader;
#[doc = "Field `AUXCNTRSTEN` writer - Enable AUXCNT Reset"]
pub type AuxcntrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HystR = crate::BitReader;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1CDIR` reader - Count direction determined by S1"]
pub type S1cdirR = crate::BitReader;
#[doc = "Field `S1CDIR` writer - Count direction determined by S1"]
pub type S1cdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Controls when the counter counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntev {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    Both = 0,
    #[doc = "1: Only counts up on up-count events."]
    Up = 1,
    #[doc = "2: Only counts down on down-count events."]
    Down = 2,
    #[doc = "3: Never counts."]
    None = 3,
}
impl From<Cntev> for u8 {
    #[inline(always)]
    fn from(variant: Cntev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntev {
    type Ux = u8;
}
impl crate::IsEnum for Cntev {}
#[doc = "Field `CNTEV` reader - Controls when the counter counts"]
pub type CntevR = crate::FieldReader<Cntev>;
impl CntevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntev {
        match self.bits {
            0 => Cntev::Both,
            1 => Cntev::Up,
            2 => Cntev::Down,
            3 => Cntev::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Cntev::Both
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Cntev::Up
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Cntev::Down
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cntev::None
    }
}
#[doc = "Field `CNTEV` writer - Controls when the counter counts"]
pub type CntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cntev, crate::Safe>;
impl<'a, REG> CntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Both)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Up)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Down)
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::None)
    }
}
#[doc = "Controls when the auxiliary counter counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Auxcntev {
    #[doc = "0: Never counts."]
    None = 0,
    #[doc = "1: Counts up on up-count events."]
    Up = 1,
    #[doc = "2: Counts up on down-count events."]
    Down = 2,
    #[doc = "3: Counts up on both up-count and down-count events."]
    Both = 3,
}
impl From<Auxcntev> for u8 {
    #[inline(always)]
    fn from(variant: Auxcntev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Auxcntev {
    type Ux = u8;
}
impl crate::IsEnum for Auxcntev {}
#[doc = "Field `AUXCNTEV` reader - Controls when the auxiliary counter counts"]
pub type AuxcntevR = crate::FieldReader<Auxcntev>;
impl AuxcntevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Auxcntev {
        match self.bits {
            0 => Auxcntev::None,
            1 => Auxcntev::Up,
            2 => Auxcntev::Down,
            3 => Auxcntev::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Auxcntev::None
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Auxcntev::Up
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Auxcntev::Down
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Auxcntev::Both
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls when the auxiliary counter counts"]
pub type AuxcntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Auxcntev, crate::Safe>;
impl<'a, REG> AuxcntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::None)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Up)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Down)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Both)
    }
}
#[doc = "Sets the mode for triggered compare and clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tccmode {
    #[doc = "0: Triggered compare and clear not enabled."]
    Disabled = 0,
    #[doc = "1: Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    Lfa = 1,
    #[doc = "2: Compare and clear performed on positive PRS edges."]
    Prs = 2,
}
impl From<Tccmode> for u8 {
    #[inline(always)]
    fn from(variant: Tccmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tccmode {
    type Ux = u8;
}
impl crate::IsEnum for Tccmode {}
#[doc = "Field `TCCMODE` reader - Sets the mode for triggered compare and clear"]
pub type TccmodeR = crate::FieldReader<Tccmode>;
impl TccmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tccmode> {
        match self.bits {
            0 => Some(Tccmode::Disabled),
            1 => Some(Tccmode::Lfa),
            2 => Some(Tccmode::Prs),
            _ => None,
        }
    }
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tccmode::Disabled
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == Tccmode::Lfa
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Tccmode::Prs
    }
}
#[doc = "Field `TCCMODE` writer - Sets the mode for triggered compare and clear"]
pub type TccmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tccmode>;
impl<'a, REG> TccmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tccmode::Disabled)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn lfa(self) -> &'a mut crate::W<REG> {
        self.variant(Tccmode::Lfa)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Tccmode::Prs)
    }
}
#[doc = "Set the LFA prescaler for triggered compare and clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tccpresc {
    #[doc = "0: Compare and clear event each LFA cycle."]
    Div1 = 0,
    #[doc = "1: Compare and clear performed on every other LFA cycle."]
    Div2 = 1,
    #[doc = "2: Compare and clear performed on every 4th LFA cycle."]
    Div4 = 2,
    #[doc = "3: Compare and clear performed on every 8th LFA cycle."]
    Div8 = 3,
}
impl From<Tccpresc> for u8 {
    #[inline(always)]
    fn from(variant: Tccpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tccpresc {
    type Ux = u8;
}
impl crate::IsEnum for Tccpresc {}
#[doc = "Field `TCCPRESC` reader - Set the LFA prescaler for triggered compare and clear"]
pub type TccprescR = crate::FieldReader<Tccpresc>;
impl TccprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tccpresc {
        match self.bits {
            0 => Tccpresc::Div1,
            1 => Tccpresc::Div2,
            2 => Tccpresc::Div4,
            3 => Tccpresc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Tccpresc::Div1
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Tccpresc::Div2
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Tccpresc::Div4
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Tccpresc::Div8
    }
}
#[doc = "Field `TCCPRESC` writer - Set the LFA prescaler for triggered compare and clear"]
pub type TccprescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tccpresc, crate::Safe>;
impl<'a, REG> TccprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div8)
    }
}
#[doc = "Triggered compare and clear compare mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcccomp {
    #[doc = "0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    Ltoe = 0,
    #[doc = "1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    Gtoe = 1,
    #[doc = "2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    Range = 2,
}
impl From<Tcccomp> for u8 {
    #[inline(always)]
    fn from(variant: Tcccomp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcccomp {
    type Ux = u8;
}
impl crate::IsEnum for Tcccomp {}
#[doc = "Field `TCCCOMP` reader - Triggered compare and clear compare mode"]
pub type TcccompR = crate::FieldReader<Tcccomp>;
impl TcccompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tcccomp> {
        match self.bits {
            0 => Some(Tcccomp::Ltoe),
            1 => Some(Tcccomp::Gtoe),
            2 => Some(Tcccomp::Range),
            _ => None,
        }
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == Tcccomp::Ltoe
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == Tcccomp::Gtoe
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == Tcccomp::Range
    }
}
#[doc = "Field `TCCCOMP` writer - Triggered compare and clear compare mode"]
pub type TcccompW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcccomp>;
impl<'a, REG> TcccompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut crate::W<REG> {
        self.variant(Tcccomp::Ltoe)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut crate::W<REG> {
        self.variant(Tcccomp::Gtoe)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(Tcccomp::Range)
    }
}
#[doc = "Field `PRSGATEEN` reader - PRS gate enable"]
pub type PrsgateenR = crate::BitReader;
#[doc = "Field `PRSGATEEN` writer - PRS gate enable"]
pub type PrsgateenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSPOL` reader - TCC PRS polarity select"]
pub type TccprspolR = crate::BitReader;
#[doc = "Field `TCCPRSPOL` writer - TCC PRS polarity select"]
pub type TccprspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TCC PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tccprssel {
    #[doc = "0: PRS Channel 0 selected."]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    Prsch5 = 5,
}
impl From<Tccprssel> for u8 {
    #[inline(always)]
    fn from(variant: Tccprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tccprssel {
    type Ux = u8;
}
impl crate::IsEnum for Tccprssel {}
#[doc = "Field `TCCPRSSEL` reader - TCC PRS Channel Select"]
pub type TccprsselR = crate::FieldReader<Tccprssel>;
impl TccprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tccprssel> {
        match self.bits {
            0 => Some(Tccprssel::Prsch0),
            1 => Some(Tccprssel::Prsch1),
            2 => Some(Tccprssel::Prsch2),
            3 => Some(Tccprssel::Prsch3),
            4 => Some(Tccprssel::Prsch4),
            5 => Some(Tccprssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Tccprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Tccprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Tccprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Tccprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Tccprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Tccprssel::Prsch5
    }
}
#[doc = "Field `TCCPRSSEL` writer - TCC PRS Channel Select"]
pub type TccprsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tccprssel>;
impl<'a, REG> TccprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch5)
    }
}
impl R {
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&self) -> CntdirR {
        CntdirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RstenR {
        RstenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AuxcntrstenR {
        AuxcntrstenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1cdirR {
        S1cdirR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CntevR {
        CntevR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AuxcntevR {
        AuxcntevR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Sets the mode for triggered compare and clear"]
    #[inline(always)]
    pub fn tccmode(&self) -> TccmodeR {
        TccmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Set the LFA prescaler for triggered compare and clear"]
    #[inline(always)]
    pub fn tccpresc(&self) -> TccprescR {
        TccprescR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Triggered compare and clear compare mode"]
    #[inline(always)]
    pub fn tcccomp(&self) -> TcccompR {
        TcccompR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - PRS gate enable"]
    #[inline(always)]
    pub fn prsgateen(&self) -> PrsgateenR {
        PrsgateenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCC PRS polarity select"]
    #[inline(always)]
    pub fn tccprspol(&self) -> TccprspolR {
        TccprspolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&self) -> TccprsselR {
        TccprsselR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&mut self) -> CntdirW<'_, CtrlSpec> {
        CntdirW::new(self, 2)
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<'_, CtrlSpec> {
        EdgeW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FiltW<'_, CtrlSpec> {
        FiltW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RstenW<'_, CtrlSpec> {
        RstenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&mut self) -> AuxcntrstenW<'_, CtrlSpec> {
        AuxcntrstenW::new(self, 6)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, CtrlSpec> {
        HystW::new(self, 8)
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline(always)]
    pub fn s1cdir(&mut self) -> S1cdirW<'_, CtrlSpec> {
        S1cdirW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline(always)]
    pub fn cntev(&mut self) -> CntevW<'_, CtrlSpec> {
        CntevW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline(always)]
    pub fn auxcntev(&mut self) -> AuxcntevW<'_, CtrlSpec> {
        AuxcntevW::new(self, 14)
    }
    #[doc = "Bits 18:19 - Sets the mode for triggered compare and clear"]
    #[inline(always)]
    pub fn tccmode(&mut self) -> TccmodeW<'_, CtrlSpec> {
        TccmodeW::new(self, 18)
    }
    #[doc = "Bits 22:23 - Set the LFA prescaler for triggered compare and clear"]
    #[inline(always)]
    pub fn tccpresc(&mut self) -> TccprescW<'_, CtrlSpec> {
        TccprescW::new(self, 22)
    }
    #[doc = "Bits 25:26 - Triggered compare and clear compare mode"]
    #[inline(always)]
    pub fn tcccomp(&mut self) -> TcccompW<'_, CtrlSpec> {
        TcccompW::new(self, 25)
    }
    #[doc = "Bit 27 - PRS gate enable"]
    #[inline(always)]
    pub fn prsgateen(&mut self) -> PrsgateenW<'_, CtrlSpec> {
        PrsgateenW::new(self, 27)
    }
    #[doc = "Bit 28 - TCC PRS polarity select"]
    #[inline(always)]
    pub fn tccprspol(&mut self) -> TccprspolW<'_, CtrlSpec> {
        TccprspolW::new(self, 28)
    }
    #[doc = "Bits 29:31 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&mut self) -> TccprsselW<'_, CtrlSpec> {
        TccprsselW::new(self, 29)
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
