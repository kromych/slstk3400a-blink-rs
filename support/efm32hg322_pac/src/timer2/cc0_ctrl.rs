#[doc = "Register `CC0_CTRL` reader"]
pub type R = crate::R<CC0_CTRL_SPEC>;
#[doc = "Register `CC0_CTRL` writer"]
pub type W = crate::W<CC0_CTRL_SPEC>;
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Compare/Capture channel turned off"]
    OFF = 0,
    #[doc = "1: Input capture"]
    INPUTCAPTURE = 1,
    #[doc = "2: Output compare"]
    OUTPUTCOMPARE = 2,
    #[doc = "3: Pulse-Width Modulation"]
    PWM = 3,
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
            0 => MODE_A::OFF,
            1 => MODE_A::INPUTCAPTURE,
            2 => MODE_A::OUTPUTCOMPARE,
            3 => MODE_A::PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == MODE_A::INPUTCAPTURE
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == MODE_A::OUTPUTCOMPARE
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OFF)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::INPUTCAPTURE)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OUTPUTCOMPARE)
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::PWM)
    }
}
#[doc = "Field `OUTINV` reader - Output Invert"]
pub type OUTINV_R = crate::BitReader;
#[doc = "Field `OUTINV` writer - Output Invert"]
pub type OUTINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COIST` reader - Compare Output Initial State"]
pub type COIST_R = crate::BitReader;
#[doc = "Field `COIST` writer - Compare Output Initial State"]
pub type COIST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CMOA_R = crate::FieldReader<CMOA_A>;
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMOA_A {
    #[doc = "0: No action on compare match"]
    NONE = 0,
    #[doc = "1: Toggle output on compare match"]
    TOGGLE = 1,
    #[doc = "2: Clear output on compare match"]
    CLEAR = 2,
    #[doc = "3: Set output on compare match"]
    SET = 3,
}
impl From<CMOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMOA_A {
    type Ux = u8;
}
impl CMOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMOA_A {
        match self.bits {
            0 => CMOA_A::NONE,
            1 => CMOA_A::TOGGLE,
            2 => CMOA_A::CLEAR,
            3 => CMOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMOA_A::NONE
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CMOA_A::TOGGLE
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMOA_A::CLEAR
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMOA_A::SET
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CMOA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CMOA_A>;
impl<'a, REG> CMOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on compare match"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA_A::NONE)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA_A::TOGGLE)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA_A::CLEAR)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(CMOA_A::SET)
    }
}
#[doc = "Field `COFOA` reader - Counter Overflow Output Action"]
pub type COFOA_R = crate::FieldReader<COFOA_A>;
#[doc = "Counter Overflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COFOA_A {
    #[doc = "0: No action on counter overflow"]
    NONE = 0,
    #[doc = "1: Toggle output on counter overflow"]
    TOGGLE = 1,
    #[doc = "2: Clear output on counter overflow"]
    CLEAR = 2,
    #[doc = "3: Set output on counter overflow"]
    SET = 3,
}
impl From<COFOA_A> for u8 {
    #[inline(always)]
    fn from(variant: COFOA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COFOA_A {
    type Ux = u8;
}
impl COFOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COFOA_A {
        match self.bits {
            0 => COFOA_A::NONE,
            1 => COFOA_A::TOGGLE,
            2 => COFOA_A::CLEAR,
            3 => COFOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == COFOA_A::NONE
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == COFOA_A::TOGGLE
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == COFOA_A::CLEAR
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == COFOA_A::SET
    }
}
#[doc = "Field `COFOA` writer - Counter Overflow Output Action"]
pub type COFOA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COFOA_A>;
impl<'a, REG> COFOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on counter overflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA_A::NONE)
    }
    #[doc = "Toggle output on counter overflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA_A::TOGGLE)
    }
    #[doc = "Clear output on counter overflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA_A::CLEAR)
    }
    #[doc = "Set output on counter overflow"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(COFOA_A::SET)
    }
}
#[doc = "Field `CUFOA` reader - Counter Underflow Output Action"]
pub type CUFOA_R = crate::FieldReader<CUFOA_A>;
#[doc = "Counter Underflow Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CUFOA_A {
    #[doc = "0: No action on counter underflow"]
    NONE = 0,
    #[doc = "1: Toggle output on counter underflow"]
    TOGGLE = 1,
    #[doc = "2: Clear output on counter underflow"]
    CLEAR = 2,
    #[doc = "3: Set output on counter underflow"]
    SET = 3,
}
impl From<CUFOA_A> for u8 {
    #[inline(always)]
    fn from(variant: CUFOA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CUFOA_A {
    type Ux = u8;
}
impl CUFOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CUFOA_A {
        match self.bits {
            0 => CUFOA_A::NONE,
            1 => CUFOA_A::TOGGLE,
            2 => CUFOA_A::CLEAR,
            3 => CUFOA_A::SET,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CUFOA_A::NONE
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == CUFOA_A::TOGGLE
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CUFOA_A::CLEAR
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CUFOA_A::SET
    }
}
#[doc = "Field `CUFOA` writer - Counter Underflow Output Action"]
pub type CUFOA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CUFOA_A>;
impl<'a, REG> CUFOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on counter underflow"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA_A::NONE)
    }
    #[doc = "Toggle output on counter underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA_A::TOGGLE)
    }
    #[doc = "Clear output on counter underflow"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA_A::CLEAR)
    }
    #[doc = "Set output on counter underflow"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(CUFOA_A::SET)
    }
}
#[doc = "Field `PRSSEL` reader - Compare/Capture Channel PRS Input Channel Selection"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "Compare/Capture Channel PRS Input Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
}
#[doc = "Field `PRSSEL` writer - Compare/Capture Channel PRS Input Channel Selection"]
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PRSSEL_A>;
impl<'a, REG> PRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
}
#[doc = "Field `INSEL` reader - Input Selection"]
pub type INSEL_R = crate::BitReader;
#[doc = "Field `INSEL` writer - Input Selection"]
pub type INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILT` reader - Digital Filter"]
pub type FILT_R = crate::BitReader;
#[doc = "Field `FILT` writer - Digital Filter"]
pub type FILT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type ICEDGE_R = crate::FieldReader<ICEDGE_A>;
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICEDGE_A {
    #[doc = "0: Rising edges detected"]
    RISING = 0,
    #[doc = "1: Falling edges detected"]
    FALLING = 1,
    #[doc = "2: Both edges detected"]
    BOTH = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    NONE = 3,
}
impl From<ICEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEDGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICEDGE_A {
    type Ux = u8;
}
impl ICEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEDGE_A {
        match self.bits {
            0 => ICEDGE_A::RISING,
            1 => ICEDGE_A::FALLING,
            2 => ICEDGE_A::BOTH,
            3 => ICEDGE_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEDGE_A::RISING
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEDGE_A::FALLING
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ICEDGE_A::BOTH
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ICEDGE_A::NONE
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type ICEDGE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ICEDGE_A>;
impl<'a, REG> ICEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE_A::RISING)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE_A::FALLING)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE_A::BOTH)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ICEDGE_A::NONE)
    }
}
#[doc = "Field `ICEVCTRL` reader - Input Capture Event Control"]
pub type ICEVCTRL_R = crate::FieldReader<ICEVCTRL_A>;
#[doc = "Input Capture Event Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICEVCTRL_A {
    #[doc = "0: PRS output pulse, interrupt flag and DMA request set on every capture"]
    EVERYEDGE = 0,
    #[doc = "1: PRS output pulse, interrupt flag and DMA request set on every second capture"]
    EVERYSECONDEDGE = 1,
    #[doc = "2: PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    RISING = 2,
    #[doc = "3: PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    FALLING = 3,
}
impl From<ICEVCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ICEVCTRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICEVCTRL_A {
    type Ux = u8;
}
impl ICEVCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICEVCTRL_A {
        match self.bits {
            0 => ICEVCTRL_A::EVERYEDGE,
            1 => ICEVCTRL_A::EVERYSECONDEDGE,
            2 => ICEVCTRL_A::RISING,
            3 => ICEVCTRL_A::FALLING,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    #[inline(always)]
    pub fn is_everyedge(&self) -> bool {
        *self == ICEVCTRL_A::EVERYEDGE
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    #[inline(always)]
    pub fn is_everysecondedge(&self) -> bool {
        *self == ICEVCTRL_A::EVERYSECONDEDGE
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ICEVCTRL_A::RISING
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ICEVCTRL_A::FALLING
    }
}
#[doc = "Field `ICEVCTRL` writer - Input Capture Event Control"]
pub type ICEVCTRL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ICEVCTRL_A>;
impl<'a, REG> ICEVCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every capture"]
    #[inline(always)]
    pub fn everyedge(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL_A::EVERYEDGE)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on every second capture"]
    #[inline(always)]
    pub fn everysecondedge(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL_A::EVERYSECONDEDGE)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on rising edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL_A::RISING)
    }
    #[doc = "PRS output pulse, interrupt flag and DMA request set on falling edge only (if ICEDGE = BOTH)"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(ICEVCTRL_A::FALLING)
    }
}
#[doc = "Field `PRSCONF` reader - PRS Configuration"]
pub type PRSCONF_R = crate::BitReader;
#[doc = "Field `PRSCONF` writer - PRS Configuration"]
pub type PRSCONF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    pub fn outinv(&self) -> OUTINV_R {
        OUTINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&self) -> COIST_R {
        COIST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CMOA_R {
        CMOA_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    pub fn cofoa(&self) -> COFOA_R {
        COFOA_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    pub fn cufoa(&self) -> CUFOA_R {
        CUFOA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> ICEDGE_R {
        ICEDGE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    pub fn icevctrl(&self) -> ICEVCTRL_R {
        ICEVCTRL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&self) -> PRSCONF_R {
        PRSCONF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CC0_CTRL_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn outinv(&mut self) -> OUTINV_W<CC0_CTRL_SPEC> {
        OUTINV_W::new(self, 2)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    #[must_use]
    pub fn coist(&mut self) -> COIST_W<CC0_CTRL_SPEC> {
        COIST_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Compare Match Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cmoa(&mut self) -> CMOA_W<CC0_CTRL_SPEC> {
        CMOA_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Counter Overflow Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cofoa(&mut self) -> COFOA_W<CC0_CTRL_SPEC> {
        COFOA_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Counter Underflow Output Action"]
    #[inline(always)]
    #[must_use]
    pub fn cufoa(&mut self) -> CUFOA_W<CC0_CTRL_SPEC> {
        CUFOA_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<CC0_CTRL_SPEC> {
        PRSSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<CC0_CTRL_SPEC> {
        INSEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Digital Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FILT_W<CC0_CTRL_SPEC> {
        FILT_W::new(self, 21)
    }
    #[doc = "Bits 24:25 - Input Capture Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn icedge(&mut self) -> ICEDGE_W<CC0_CTRL_SPEC> {
        ICEDGE_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Input Capture Event Control"]
    #[inline(always)]
    #[must_use]
    pub fn icevctrl(&mut self) -> ICEVCTRL_W<CC0_CTRL_SPEC> {
        ICEVCTRL_W::new(self, 26)
    }
    #[doc = "Bit 28 - PRS Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prsconf(&mut self) -> PRSCONF_W<CC0_CTRL_SPEC> {
        PRSCONF_W::new(self, 28)
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
#[doc = "CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_CTRL_SPEC;
impl crate::RegisterSpec for CC0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ctrl::R`](R) reader structure"]
impl crate::Readable for CC0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc0_ctrl::W`](W) writer structure"]
impl crate::Writable for CC0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC0_CTRL to value 0"]
impl crate::Resettable for CC0_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
