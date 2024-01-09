#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `MODE` reader - Mode Select"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: The module is disabled."]
    DISABLE = 0,
    #[doc = "1: Single input LFACLK oversampling mode (available in EM0-EM2)."]
    OVSSINGLE = 1,
    #[doc = "2: Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE = 2,
    #[doc = "3: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::DISABLE,
            1 => MODE_A::OVSSINGLE,
            2 => MODE_A::EXTCLKSINGLE,
            3 => MODE_A::EXTCLKQUAD,
            _ => unreachable!(),
        }
    }
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODE_A::OVSSINGLE
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODE_A::EXTCLKSINGLE
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODE_A::EXTCLKQUAD
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM2)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OVSSINGLE)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::EXTCLKSINGLE)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::EXTCLKQUAD)
    }
}
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_R = crate::BitReader;
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EDGE_R = crate::BitReader;
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILT` reader - Enable Digital Pulse Width Filter"]
pub type FILT_R = crate::BitReader;
#[doc = "Field `FILT` writer - Enable Digital Pulse Width Filter"]
pub type FILT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTEN` reader - Enable PCNT Clock Domain Reset"]
pub type RSTEN_R = crate::BitReader;
#[doc = "Field `RSTEN` writer - Enable PCNT Clock Domain Reset"]
pub type RSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXCNTRSTEN` reader - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_R = crate::BitReader;
#[doc = "Field `AUXCNTRSTEN` writer - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HYST_R = crate::BitReader;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HYST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1CDIR` reader - Count direction determined by S1"]
pub type S1CDIR_R = crate::BitReader;
#[doc = "Field `S1CDIR` writer - Count direction determined by S1"]
pub type S1CDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTEV` reader - Controls when the counter counts"]
pub type CNTEV_R = crate::FieldReader<CNTEV_A>;
#[doc = "Controls when the counter counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTEV_A {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    BOTH = 0,
    #[doc = "1: Only counts up on up-count events."]
    UP = 1,
    #[doc = "2: Only counts down on down-count events."]
    DOWN = 2,
    #[doc = "3: Never counts."]
    NONE = 3,
}
impl From<CNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTEV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNTEV_A {
    type Ux = u8;
}
impl CNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTEV_A {
        match self.bits {
            0 => CNTEV_A::BOTH,
            1 => CNTEV_A::UP,
            2 => CNTEV_A::DOWN,
            3 => CNTEV_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CNTEV_A::BOTH
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTEV_A::UP
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTEV_A::DOWN
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CNTEV_A::NONE
    }
}
#[doc = "Field `CNTEV` writer - Controls when the counter counts"]
pub type CNTEV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CNTEV_A>;
impl<'a, REG> CNTEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::BOTH)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::UP)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::DOWN)
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::NONE)
    }
}
#[doc = "Field `AUXCNTEV` reader - Controls when the auxiliary counter counts"]
pub type AUXCNTEV_R = crate::FieldReader<AUXCNTEV_A>;
#[doc = "Controls when the auxiliary counter counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUXCNTEV_A {
    #[doc = "0: Never counts."]
    NONE = 0,
    #[doc = "1: Counts up on up-count events."]
    UP = 1,
    #[doc = "2: Counts up on down-count events."]
    DOWN = 2,
    #[doc = "3: Counts up on both up-count and down-count events."]
    BOTH = 3,
}
impl From<AUXCNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXCNTEV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AUXCNTEV_A {
    type Ux = u8;
}
impl AUXCNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUXCNTEV_A {
        match self.bits {
            0 => AUXCNTEV_A::NONE,
            1 => AUXCNTEV_A::UP,
            2 => AUXCNTEV_A::DOWN,
            3 => AUXCNTEV_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEV_A::NONE
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEV_A::UP
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEV_A::DOWN
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEV_A::BOTH
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls when the auxiliary counter counts"]
pub type AUXCNTEV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AUXCNTEV_A>;
impl<'a, REG> AUXCNTEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::NONE)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::UP)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::DOWN)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::BOTH)
    }
}
#[doc = "Field `TCCMODE` reader - Sets the mode for triggered compare and clear"]
pub type TCCMODE_R = crate::FieldReader<TCCMODE_A>;
#[doc = "Sets the mode for triggered compare and clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCMODE_A {
    #[doc = "0: Triggered compare and clear not enabled."]
    DISABLED = 0,
    #[doc = "1: Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    LFA = 1,
    #[doc = "2: Compare and clear performed on positive PRS edges."]
    PRS = 2,
}
impl From<TCCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCMODE_A {
    type Ux = u8;
}
impl TCCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCMODE_A> {
        match self.bits {
            0 => Some(TCCMODE_A::DISABLED),
            1 => Some(TCCMODE_A::LFA),
            2 => Some(TCCMODE_A::PRS),
            _ => None,
        }
    }
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCCMODE_A::DISABLED
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == TCCMODE_A::LFA
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TCCMODE_A::PRS
    }
}
#[doc = "Field `TCCMODE` writer - Sets the mode for triggered compare and clear"]
pub type TCCMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCMODE_A>;
impl<'a, REG> TCCMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE_A::DISABLED)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn lfa(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE_A::LFA)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE_A::PRS)
    }
}
#[doc = "Field `TCCPRESC` reader - Set the LFA prescaler for triggered compare and clear"]
pub type TCCPRESC_R = crate::FieldReader<TCCPRESC_A>;
#[doc = "Set the LFA prescaler for triggered compare and clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRESC_A {
    #[doc = "0: Compare and clear event each LFA cycle."]
    DIV1 = 0,
    #[doc = "1: Compare and clear performed on every other LFA cycle."]
    DIV2 = 1,
    #[doc = "2: Compare and clear performed on every 4th LFA cycle."]
    DIV4 = 2,
    #[doc = "3: Compare and clear performed on every 8th LFA cycle."]
    DIV8 = 3,
}
impl From<TCCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCPRESC_A {
    type Ux = u8;
}
impl TCCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCCPRESC_A {
        match self.bits {
            0 => TCCPRESC_A::DIV1,
            1 => TCCPRESC_A::DIV2,
            2 => TCCPRESC_A::DIV4,
            3 => TCCPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == TCCPRESC_A::DIV1
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == TCCPRESC_A::DIV2
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TCCPRESC_A::DIV4
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == TCCPRESC_A::DIV8
    }
}
#[doc = "Field `TCCPRESC` writer - Set the LFA prescaler for triggered compare and clear"]
pub type TCCPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TCCPRESC_A>;
impl<'a, REG> TCCPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV8)
    }
}
#[doc = "Field `TCCCOMP` reader - Triggered compare and clear compare mode"]
pub type TCCCOMP_R = crate::FieldReader<TCCCOMP_A>;
#[doc = "Triggered compare and clear compare mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCCOMP_A {
    #[doc = "0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    LTOE = 0,
    #[doc = "1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    GTOE = 1,
    #[doc = "2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    RANGE = 2,
}
impl From<TCCCOMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCCOMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCCOMP_A {
    type Ux = u8;
}
impl TCCCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCCOMP_A> {
        match self.bits {
            0 => Some(TCCCOMP_A::LTOE),
            1 => Some(TCCCOMP_A::GTOE),
            2 => Some(TCCCOMP_A::RANGE),
            _ => None,
        }
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == TCCCOMP_A::LTOE
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == TCCCOMP_A::GTOE
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == TCCCOMP_A::RANGE
    }
}
#[doc = "Field `TCCCOMP` writer - Triggered compare and clear compare mode"]
pub type TCCCOMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCCOMP_A>;
impl<'a, REG> TCCCOMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP_A::LTOE)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP_A::GTOE)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP_A::RANGE)
    }
}
#[doc = "Field `PRSGATEEN` reader - PRS gate enable"]
pub type PRSGATEEN_R = crate::BitReader;
#[doc = "Field `PRSGATEEN` writer - PRS gate enable"]
pub type PRSGATEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSPOL` reader - TCC PRS polarity select"]
pub type TCCPRSPOL_R = crate::BitReader;
#[doc = "Field `TCCPRSPOL` writer - TCC PRS polarity select"]
pub type TCCPRSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSSEL` reader - TCC PRS Channel Select"]
pub type TCCPRSSEL_R = crate::FieldReader<TCCPRSSEL_A>;
#[doc = "TCC PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
}
impl From<TCCPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCPRSSEL_A {
    type Ux = u8;
}
impl TCCPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCPRSSEL_A> {
        match self.bits {
            0 => Some(TCCPRSSEL_A::PRSCH0),
            1 => Some(TCCPRSSEL_A::PRSCH1),
            2 => Some(TCCPRSSEL_A::PRSCH2),
            3 => Some(TCCPRSSEL_A::PRSCH3),
            4 => Some(TCCPRSSEL_A::PRSCH4),
            5 => Some(TCCPRSSEL_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH5
    }
}
#[doc = "Field `TCCPRSSEL` writer - TCC PRS Channel Select"]
pub type TCCPRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TCCPRSSEL_A>;
impl<'a, REG> TCCPRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH5)
    }
}
impl R {
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&self) -> CNTDIR_R {
        CNTDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AUXCNTRSTEN_R {
        AUXCNTRSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1CDIR_R {
        S1CDIR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CNTEV_R {
        CNTEV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AUXCNTEV_R {
        AUXCNTEV_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Sets the mode for triggered compare and clear"]
    #[inline(always)]
    pub fn tccmode(&self) -> TCCMODE_R {
        TCCMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Set the LFA prescaler for triggered compare and clear"]
    #[inline(always)]
    pub fn tccpresc(&self) -> TCCPRESC_R {
        TCCPRESC_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Triggered compare and clear compare mode"]
    #[inline(always)]
    pub fn tcccomp(&self) -> TCCCOMP_R {
        TCCCOMP_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - PRS gate enable"]
    #[inline(always)]
    pub fn prsgateen(&self) -> PRSGATEEN_R {
        PRSGATEEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCC PRS polarity select"]
    #[inline(always)]
    pub fn tccprspol(&self) -> TCCPRSPOL_R {
        TCCPRSPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&self) -> TCCPRSSEL_R {
        TCCPRSSEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRL_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    #[must_use]
    pub fn cntdir(&mut self) -> CNTDIR_W<CTRL_SPEC> {
        CNTDIR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<CTRL_SPEC> {
        EDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FILT_W<CTRL_SPEC> {
        FILT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RSTEN_W<CTRL_SPEC> {
        RSTEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntrsten(&mut self) -> AUXCNTRSTEN_W<CTRL_SPEC> {
        AUXCNTRSTEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<CTRL_SPEC> {
        HYST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Count direction determined by S1"]
    #[inline(always)]
    #[must_use]
    pub fn s1cdir(&mut self) -> S1CDIR_W<CTRL_SPEC> {
        S1CDIR_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Controls when the counter counts"]
    #[inline(always)]
    #[must_use]
    pub fn cntev(&mut self) -> CNTEV_W<CTRL_SPEC> {
        CNTEV_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - Controls when the auxiliary counter counts"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntev(&mut self) -> AUXCNTEV_W<CTRL_SPEC> {
        AUXCNTEV_W::new(self, 14)
    }
    #[doc = "Bits 18:19 - Sets the mode for triggered compare and clear"]
    #[inline(always)]
    #[must_use]
    pub fn tccmode(&mut self) -> TCCMODE_W<CTRL_SPEC> {
        TCCMODE_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - Set the LFA prescaler for triggered compare and clear"]
    #[inline(always)]
    #[must_use]
    pub fn tccpresc(&mut self) -> TCCPRESC_W<CTRL_SPEC> {
        TCCPRESC_W::new(self, 22)
    }
    #[doc = "Bits 25:26 - Triggered compare and clear compare mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcccomp(&mut self) -> TCCCOMP_W<CTRL_SPEC> {
        TCCCOMP_W::new(self, 25)
    }
    #[doc = "Bit 27 - PRS gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsgateen(&mut self) -> PRSGATEEN_W<CTRL_SPEC> {
        PRSGATEEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - TCC PRS polarity select"]
    #[inline(always)]
    #[must_use]
    pub fn tccprspol(&mut self) -> TCCPRSPOL_W<CTRL_SPEC> {
        TCCPRSPOL_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - TCC PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn tccprssel(&mut self) -> TCCPRSSEL_W<CTRL_SPEC> {
        TCCPRSSEL_W::new(self, 29)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
