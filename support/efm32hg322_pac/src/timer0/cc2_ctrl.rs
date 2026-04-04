#[doc = "Register `CC2_CTRL` reader"]
pub type R = crate::R<Cc2CtrlSpec>;
#[doc = "Register `CC2_CTRL` writer"]
pub type W = crate::W<Cc2CtrlSpec>;
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Compare/Capture channel turned off"]
    Off = 0,
    #[doc = "1: Input capture"]
    Inputcapture = 1,
    #[doc = "2: Output compare"]
    Outputcompare = 2,
    #[doc = "3: Pulse-Width Modulation"]
    Pwm = 3,
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
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Off,
            1 => Mode::Inputcapture,
            2 => Mode::Outputcompare,
            3 => Mode::Pwm,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode::Off
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == Mode::Inputcapture
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == Mode::Outputcompare
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Mode::Pwm
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Off)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Inputcapture)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Outputcompare)
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Pwm)
    }
}
#[doc = "Field `OUTINV` reader - Output Invert"]
pub type OutinvR = crate::BitReader;
#[doc = "Field `OUTINV` writer - Output Invert"]
pub type OutinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COIST` reader - Compare Output Initial State"]
pub type CoistR = crate::BitReader;
#[doc = "Field `COIST` writer - Compare Output Initial State"]
pub type CoistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmoa {
    #[doc = "0: No action on compare match"]
    None = 0,
    #[doc = "1: Toggle output on compare match"]
    Toggle = 1,
    #[doc = "2: Clear output on compare match"]
    Clear = 2,
    #[doc = "3: Set output on compare match"]
    Set = 3,
}
impl From<Cmoa> for u8 {
    #[inline(always)]
    fn from(variant: Cmoa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmoa {
    type Ux = u8;
}
impl crate::IsEnum for Cmoa {}
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CmoaR = crate::FieldReader<Cmoa>;
impl CmoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmoa {
        match self.bits {
            0 => Cmoa::None,
            1 => Cmoa::Toggle,
            2 => Cmoa::Clear,
            3 => Cmoa::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cmoa::None
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cmoa::Toggle
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cmoa::Clear
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cmoa::Set
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CmoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmoa, crate::Safe>;
impl<'a, REG> CmoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::None)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Toggle)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Clear)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Set)
    }
}
#[doc = "Counter Overflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cofoa {
    #[doc = "0: No action on counter overflow"]
    None = 0,
    #[doc = "1: Toggle output on counter overflow"]
    Toggle = 1,
    #[doc = "2: Clear output on counter overflow"]
    Clear = 2,
    #[doc = "3: Set output on counter overflow"]
    Set = 3,
}
impl From<Cofoa> for u8 {
    #[inline(always)]
    fn from(variant: Cofoa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cofoa {
    type Ux = u8;
}
impl crate::IsEnum for Cofoa {}
#[doc = "Field `COFOA` reader - Counter Overflow Output Action"]
pub type CofoaR = crate::FieldReader<Cofoa>;
impl CofoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cofoa {
        match self.bits {
            0 => Cofoa::None,
            1 => Cofoa::Toggle,
            2 => Cofoa::Clear,
            3 => Cofoa::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cofoa::None
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cofoa::Toggle
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cofoa::Clear
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cofoa::Set
    }
}
#[doc = "Field `COFOA` writer - Counter Overflow Output Action"]
pub type CofoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cofoa, crate::Safe>;
impl<'a, REG> CofoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cofoa::None)
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cofoa::Toggle)
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cofoa::Clear)
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cofoa::Set)
    }
}
#[doc = "Counter Underflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cufoa {
    #[doc = "0: No action on counter underflow"]
    None = 0,
    #[doc = "1: Toggle output on counter underflow"]
    Toggle = 1,
    #[doc = "2: Clear output on counter underflow"]
    Clear = 2,
    #[doc = "3: Set output on counter underflow"]
    Set = 3,
}
impl From<Cufoa> for u8 {
    #[inline(always)]
    fn from(variant: Cufoa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cufoa {
    type Ux = u8;
}
impl crate::IsEnum for Cufoa {}
#[doc = "Field `CUFOA` reader - Counter Underflow Output Action"]
pub type CufoaR = crate::FieldReader<Cufoa>;
impl CufoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cufoa {
        match self.bits {
            0 => Cufoa::None,
            1 => Cufoa::Toggle,
            2 => Cufoa::Clear,
            3 => Cufoa::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cufoa::None
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cufoa::Toggle
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cufoa::Clear
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cufoa::Set
    }
}
#[doc = "Field `CUFOA` writer - Counter Underflow Output Action"]
pub type CufoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cufoa, crate::Safe>;
impl<'a, REG> CufoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cufoa::None)
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cufoa::Toggle)
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cufoa::Clear)
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cufoa::Set)
    }
}
#[doc = "Compare/Capture Channel PRS Input Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
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
#[doc = "Field `PRSSEL` reader - Compare/Capture Channel PRS Input Channel Selection"]
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
}
#[doc = "Field `PRSSEL` writer - Compare/Capture Channel PRS Input Channel Selection"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
}
#[doc = "Field `INSEL` reader - Input Selection"]
pub type InselR = crate::BitReader;
#[doc = "Field `INSEL` writer - Input Selection"]
pub type InselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILT` reader - Digital Filter"]
pub type FiltR = crate::BitReader;
#[doc = "Field `FILT` writer - Digital Filter"]
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Icedge {
    #[doc = "0: Rising edges detected"]
    Rising = 0,
    #[doc = "1: Falling edges detected"]
    Falling = 1,
    #[doc = "2: Both edges detected"]
    Both = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    None = 3,
}
impl From<Icedge> for u8 {
    #[inline(always)]
    fn from(variant: Icedge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Icedge {
    type Ux = u8;
}
impl crate::IsEnum for Icedge {}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type IcedgeR = crate::FieldReader<Icedge>;
impl IcedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icedge {
        match self.bits {
            0 => Icedge::Rising,
            1 => Icedge::Falling,
            2 => Icedge::Both,
            3 => Icedge::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Icedge::Rising
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Icedge::Falling
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Icedge::Both
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Icedge::None
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type IcedgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Icedge, crate::Safe>;
impl<'a, REG> IcedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Rising)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Falling)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Both)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::None)
    }
}
#[doc = "Input Capture Event Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Icevctrl {
    #[doc = "0: PRS output pulse, interrupt flag and DMA request set on every capture"]
    Everyedge = 0,
    #[doc = "1: PRS output pulse, interrupt flag and DMA request set on every second capture"]
    Everysecondedge = 1,
    #[doc = "2: PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    Rising = 2,
    #[doc = "3: PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    Falling = 3,
}
impl From<Icevctrl> for u8 {
    #[inline(always)]
    fn from(variant: Icevctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Icevctrl {
    type Ux = u8;
}
impl crate::IsEnum for Icevctrl {}
#[doc = "Field `ICEVCTRL` reader - Input Capture Event Control"]
pub type IcevctrlR = crate::FieldReader<Icevctrl>;
impl IcevctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icevctrl {
        match self.bits {
            0 => Icevctrl::Everyedge,
            1 => Icevctrl::Everysecondedge,
            2 => Icevctrl::Rising,
            3 => Icevctrl::Falling,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    #[inline(always)]
    pub fn is_everyedge(&self) -> bool {
        *self == Icevctrl::Everyedge
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    #[inline(always)]
    pub fn is_everysecondedge(&self) -> bool {
        *self == Icevctrl::Everysecondedge
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Icevctrl::Rising
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Icevctrl::Falling
    }
}
#[doc = "Field `ICEVCTRL` writer - Input Capture Event Control"]
pub type IcevctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Icevctrl, crate::Safe>;
impl<'a, REG> IcevctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    #[inline(always)]
    pub fn everyedge(self) -> &'a mut crate::W<REG> {
        self.variant(Icevctrl::Everyedge)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    #[inline(always)]
    pub fn everysecondedge(self) -> &'a mut crate::W<REG> {
        self.variant(Icevctrl::Everysecondedge)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Icevctrl::Rising)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Icevctrl::Falling)
    }
}
#[doc = "Field `PRSCONF` reader - PRS Configuration"]
pub type PrsconfR = crate::BitReader;
#[doc = "Field `PRSCONF` writer - PRS Configuration"]
pub type PrsconfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    pub fn outinv(&self) -> OutinvR {
        OutinvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&self) -> CoistR {
        CoistR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CmoaR {
        CmoaR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    pub fn cofoa(&self) -> CofoaR {
        CofoaR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    pub fn cufoa(&self) -> CufoaR {
        CufoaR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline(always)]
    pub fn insel(&self) -> InselR {
        InselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> IcedgeR {
        IcedgeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    pub fn icevctrl(&self) -> IcevctrlR {
        IcevctrlR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&self) -> PrsconfR {
        PrsconfR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cc2CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    pub fn outinv(&mut self) -> OutinvW<'_, Cc2CtrlSpec> {
        OutinvW::new(self, 2)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&mut self) -> CoistW<'_, Cc2CtrlSpec> {
        CoistW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&mut self) -> CmoaW<'_, Cc2CtrlSpec> {
        CmoaW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    pub fn cofoa(&mut self) -> CofoaW<'_, Cc2CtrlSpec> {
        CofoaW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    pub fn cufoa(&mut self) -> CufoaW<'_, Cc2CtrlSpec> {
        CufoaW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, Cc2CtrlSpec> {
        PrsselW::new(self, 16)
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> InselW<'_, Cc2CtrlSpec> {
        InselW::new(self, 20)
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FiltW<'_, Cc2CtrlSpec> {
        FiltW::new(self, 21)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&mut self) -> IcedgeW<'_, Cc2CtrlSpec> {
        IcedgeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    pub fn icevctrl(&mut self) -> IcevctrlW<'_, Cc2CtrlSpec> {
        IcevctrlW::new(self, 26)
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&mut self) -> PrsconfW<'_, Cc2CtrlSpec> {
        PrsconfW::new(self, 28)
    }
}
#[doc = "CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2CtrlSpec;
impl crate::RegisterSpec for Cc2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ctrl::R`](R) reader structure"]
impl crate::Readable for Cc2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cc2_ctrl::W`](W) writer structure"]
impl crate::Writable for Cc2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC2_CTRL to value 0"]
impl crate::Resettable for Cc2CtrlSpec {}
