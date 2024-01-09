#[doc = "Register `EXTIPSELL` reader"]
pub type R = crate::R<EXTIPSELL_SPEC>;
#[doc = "Register `EXTIPSELL` writer"]
pub type W = crate::W<EXTIPSELL_SPEC>;
#[doc = "Field `EXTIPSEL0` reader - External Interrupt 0 Port Select"]
pub type EXTIPSEL0_R = crate::FieldReader<EXTIPSEL0_A>;
#[doc = "External Interrupt 0 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL0_A {
    #[doc = "0: Port A pin 0 selected for external interrupt 0"]
    PORTA = 0,
    #[doc = "1: Port B pin 0 selected for external interrupt 0"]
    PORTB = 1,
    #[doc = "2: Port C pin 0 selected for external interrupt 0"]
    PORTC = 2,
    #[doc = "3: Port D pin 0 selected for external interrupt 0"]
    PORTD = 3,
    #[doc = "4: Port E pin 0 selected for external interrupt 0"]
    PORTE = 4,
    #[doc = "5: Port F pin 0 selected for external interrupt 0"]
    PORTF = 5,
}
impl From<EXTIPSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL0_A {
    type Ux = u8;
}
impl EXTIPSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL0_A> {
        match self.bits {
            0 => Some(EXTIPSEL0_A::PORTA),
            1 => Some(EXTIPSEL0_A::PORTB),
            2 => Some(EXTIPSEL0_A::PORTC),
            3 => Some(EXTIPSEL0_A::PORTD),
            4 => Some(EXTIPSEL0_A::PORTE),
            5 => Some(EXTIPSEL0_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL0_A::PORTA
    }
    #[doc = "Port B pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL0_A::PORTB
    }
    #[doc = "Port C pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL0_A::PORTC
    }
    #[doc = "Port D pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL0_A::PORTD
    }
    #[doc = "Port E pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL0_A::PORTE
    }
    #[doc = "Port F pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL0_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL0` writer - External Interrupt 0 Port Select"]
pub type EXTIPSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL0_A>;
impl<'a, REG> EXTIPSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0_A::PORTA)
    }
    #[doc = "Port B pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0_A::PORTB)
    }
    #[doc = "Port C pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0_A::PORTC)
    }
    #[doc = "Port D pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0_A::PORTD)
    }
    #[doc = "Port E pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0_A::PORTE)
    }
    #[doc = "Port F pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL0_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL1` reader - External Interrupt 1 Port Select"]
pub type EXTIPSEL1_R = crate::FieldReader<EXTIPSEL1_A>;
#[doc = "External Interrupt 1 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL1_A {
    #[doc = "0: Port A pin 1 selected for external interrupt 1"]
    PORTA = 0,
    #[doc = "1: Port B pin 1 selected for external interrupt 1"]
    PORTB = 1,
    #[doc = "2: Port C pin 1 selected for external interrupt 1"]
    PORTC = 2,
    #[doc = "3: Port D pin 1 selected for external interrupt 1"]
    PORTD = 3,
    #[doc = "4: Port E pin 1 selected for external interrupt 1"]
    PORTE = 4,
    #[doc = "5: Port F pin 1 selected for external interrupt 1"]
    PORTF = 5,
}
impl From<EXTIPSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL1_A {
    type Ux = u8;
}
impl EXTIPSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL1_A> {
        match self.bits {
            0 => Some(EXTIPSEL1_A::PORTA),
            1 => Some(EXTIPSEL1_A::PORTB),
            2 => Some(EXTIPSEL1_A::PORTC),
            3 => Some(EXTIPSEL1_A::PORTD),
            4 => Some(EXTIPSEL1_A::PORTE),
            5 => Some(EXTIPSEL1_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL1_A::PORTA
    }
    #[doc = "Port B pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL1_A::PORTB
    }
    #[doc = "Port C pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL1_A::PORTC
    }
    #[doc = "Port D pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL1_A::PORTD
    }
    #[doc = "Port E pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL1_A::PORTE
    }
    #[doc = "Port F pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL1_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL1` writer - External Interrupt 1 Port Select"]
pub type EXTIPSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL1_A>;
impl<'a, REG> EXTIPSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1_A::PORTA)
    }
    #[doc = "Port B pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1_A::PORTB)
    }
    #[doc = "Port C pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1_A::PORTC)
    }
    #[doc = "Port D pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1_A::PORTD)
    }
    #[doc = "Port E pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1_A::PORTE)
    }
    #[doc = "Port F pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL1_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL2` reader - External Interrupt 2 Port Select"]
pub type EXTIPSEL2_R = crate::FieldReader<EXTIPSEL2_A>;
#[doc = "External Interrupt 2 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL2_A {
    #[doc = "0: Port A pin 2 selected for external interrupt 2"]
    PORTA = 0,
    #[doc = "1: Port B pin 2 selected for external interrupt 2"]
    PORTB = 1,
    #[doc = "2: Port C pin 2 selected for external interrupt 2"]
    PORTC = 2,
    #[doc = "3: Port D pin 2 selected for external interrupt 2"]
    PORTD = 3,
    #[doc = "4: Port E pin 2 selected for external interrupt 2"]
    PORTE = 4,
    #[doc = "5: Port F pin 2 selected for external interrupt 2"]
    PORTF = 5,
}
impl From<EXTIPSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL2_A {
    type Ux = u8;
}
impl EXTIPSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL2_A> {
        match self.bits {
            0 => Some(EXTIPSEL2_A::PORTA),
            1 => Some(EXTIPSEL2_A::PORTB),
            2 => Some(EXTIPSEL2_A::PORTC),
            3 => Some(EXTIPSEL2_A::PORTD),
            4 => Some(EXTIPSEL2_A::PORTE),
            5 => Some(EXTIPSEL2_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL2_A::PORTA
    }
    #[doc = "Port B pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL2_A::PORTB
    }
    #[doc = "Port C pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL2_A::PORTC
    }
    #[doc = "Port D pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL2_A::PORTD
    }
    #[doc = "Port E pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL2_A::PORTE
    }
    #[doc = "Port F pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL2_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL2` writer - External Interrupt 2 Port Select"]
pub type EXTIPSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL2_A>;
impl<'a, REG> EXTIPSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2_A::PORTA)
    }
    #[doc = "Port B pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2_A::PORTB)
    }
    #[doc = "Port C pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2_A::PORTC)
    }
    #[doc = "Port D pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2_A::PORTD)
    }
    #[doc = "Port E pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2_A::PORTE)
    }
    #[doc = "Port F pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL2_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL3` reader - External Interrupt 3 Port Select"]
pub type EXTIPSEL3_R = crate::FieldReader<EXTIPSEL3_A>;
#[doc = "External Interrupt 3 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL3_A {
    #[doc = "0: Port A pin 3 selected for external interrupt 3"]
    PORTA = 0,
    #[doc = "1: Port B pin 3 selected for external interrupt 3"]
    PORTB = 1,
    #[doc = "2: Port C pin 3 selected for external interrupt 3"]
    PORTC = 2,
    #[doc = "3: Port D pin 3 selected for external interrupt 3"]
    PORTD = 3,
    #[doc = "4: Port E pin 3 selected for external interrupt 3"]
    PORTE = 4,
    #[doc = "5: Port F pin 3 selected for external interrupt 3"]
    PORTF = 5,
}
impl From<EXTIPSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL3_A {
    type Ux = u8;
}
impl EXTIPSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL3_A> {
        match self.bits {
            0 => Some(EXTIPSEL3_A::PORTA),
            1 => Some(EXTIPSEL3_A::PORTB),
            2 => Some(EXTIPSEL3_A::PORTC),
            3 => Some(EXTIPSEL3_A::PORTD),
            4 => Some(EXTIPSEL3_A::PORTE),
            5 => Some(EXTIPSEL3_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL3_A::PORTA
    }
    #[doc = "Port B pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL3_A::PORTB
    }
    #[doc = "Port C pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL3_A::PORTC
    }
    #[doc = "Port D pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL3_A::PORTD
    }
    #[doc = "Port E pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL3_A::PORTE
    }
    #[doc = "Port F pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL3_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL3` writer - External Interrupt 3 Port Select"]
pub type EXTIPSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL3_A>;
impl<'a, REG> EXTIPSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3_A::PORTA)
    }
    #[doc = "Port B pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3_A::PORTB)
    }
    #[doc = "Port C pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3_A::PORTC)
    }
    #[doc = "Port D pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3_A::PORTD)
    }
    #[doc = "Port E pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3_A::PORTE)
    }
    #[doc = "Port F pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL3_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL4` reader - External Interrupt 4 Port Select"]
pub type EXTIPSEL4_R = crate::FieldReader<EXTIPSEL4_A>;
#[doc = "External Interrupt 4 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL4_A {
    #[doc = "0: Port A pin 4 selected for external interrupt 4"]
    PORTA = 0,
    #[doc = "1: Port B pin 4 selected for external interrupt 4"]
    PORTB = 1,
    #[doc = "2: Port C pin 4 selected for external interrupt 4"]
    PORTC = 2,
    #[doc = "3: Port D pin 4 selected for external interrupt 4"]
    PORTD = 3,
    #[doc = "4: Port E pin 4 selected for external interrupt 4"]
    PORTE = 4,
    #[doc = "5: Port F pin 4 selected for external interrupt 4"]
    PORTF = 5,
}
impl From<EXTIPSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL4_A {
    type Ux = u8;
}
impl EXTIPSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL4_A> {
        match self.bits {
            0 => Some(EXTIPSEL4_A::PORTA),
            1 => Some(EXTIPSEL4_A::PORTB),
            2 => Some(EXTIPSEL4_A::PORTC),
            3 => Some(EXTIPSEL4_A::PORTD),
            4 => Some(EXTIPSEL4_A::PORTE),
            5 => Some(EXTIPSEL4_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL4_A::PORTA
    }
    #[doc = "Port B pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL4_A::PORTB
    }
    #[doc = "Port C pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL4_A::PORTC
    }
    #[doc = "Port D pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL4_A::PORTD
    }
    #[doc = "Port E pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL4_A::PORTE
    }
    #[doc = "Port F pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL4_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL4` writer - External Interrupt 4 Port Select"]
pub type EXTIPSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL4_A>;
impl<'a, REG> EXTIPSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4_A::PORTA)
    }
    #[doc = "Port B pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4_A::PORTB)
    }
    #[doc = "Port C pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4_A::PORTC)
    }
    #[doc = "Port D pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4_A::PORTD)
    }
    #[doc = "Port E pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4_A::PORTE)
    }
    #[doc = "Port F pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL4_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL5` reader - External Interrupt 5 Port Select"]
pub type EXTIPSEL5_R = crate::FieldReader<EXTIPSEL5_A>;
#[doc = "External Interrupt 5 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL5_A {
    #[doc = "0: Port A pin 5 selected for external interrupt 5"]
    PORTA = 0,
    #[doc = "1: Port B pin 5 selected for external interrupt 5"]
    PORTB = 1,
    #[doc = "2: Port C pin 5 selected for external interrupt 5"]
    PORTC = 2,
    #[doc = "3: Port D pin 5 selected for external interrupt 5"]
    PORTD = 3,
    #[doc = "4: Port E pin 5 selected for external interrupt 5"]
    PORTE = 4,
    #[doc = "5: Port F pin 5 selected for external interrupt 5"]
    PORTF = 5,
}
impl From<EXTIPSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL5_A {
    type Ux = u8;
}
impl EXTIPSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL5_A> {
        match self.bits {
            0 => Some(EXTIPSEL5_A::PORTA),
            1 => Some(EXTIPSEL5_A::PORTB),
            2 => Some(EXTIPSEL5_A::PORTC),
            3 => Some(EXTIPSEL5_A::PORTD),
            4 => Some(EXTIPSEL5_A::PORTE),
            5 => Some(EXTIPSEL5_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL5_A::PORTA
    }
    #[doc = "Port B pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL5_A::PORTB
    }
    #[doc = "Port C pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL5_A::PORTC
    }
    #[doc = "Port D pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL5_A::PORTD
    }
    #[doc = "Port E pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL5_A::PORTE
    }
    #[doc = "Port F pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL5_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL5` writer - External Interrupt 5 Port Select"]
pub type EXTIPSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL5_A>;
impl<'a, REG> EXTIPSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5_A::PORTA)
    }
    #[doc = "Port B pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5_A::PORTB)
    }
    #[doc = "Port C pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5_A::PORTC)
    }
    #[doc = "Port D pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5_A::PORTD)
    }
    #[doc = "Port E pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5_A::PORTE)
    }
    #[doc = "Port F pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL5_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL6` reader - External Interrupt 6 Port Select"]
pub type EXTIPSEL6_R = crate::FieldReader<EXTIPSEL6_A>;
#[doc = "External Interrupt 6 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL6_A {
    #[doc = "0: Port A pin 6 selected for external interrupt 6"]
    PORTA = 0,
    #[doc = "1: Port B pin 6 selected for external interrupt 6"]
    PORTB = 1,
    #[doc = "2: Port C pin 6 selected for external interrupt 6"]
    PORTC = 2,
    #[doc = "3: Port D pin 6 selected for external interrupt 6"]
    PORTD = 3,
    #[doc = "4: Port E pin 6 selected for external interrupt 6"]
    PORTE = 4,
    #[doc = "5: Port F pin 6 selected for external interrupt 6"]
    PORTF = 5,
}
impl From<EXTIPSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL6_A {
    type Ux = u8;
}
impl EXTIPSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL6_A> {
        match self.bits {
            0 => Some(EXTIPSEL6_A::PORTA),
            1 => Some(EXTIPSEL6_A::PORTB),
            2 => Some(EXTIPSEL6_A::PORTC),
            3 => Some(EXTIPSEL6_A::PORTD),
            4 => Some(EXTIPSEL6_A::PORTE),
            5 => Some(EXTIPSEL6_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL6_A::PORTA
    }
    #[doc = "Port B pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL6_A::PORTB
    }
    #[doc = "Port C pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL6_A::PORTC
    }
    #[doc = "Port D pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL6_A::PORTD
    }
    #[doc = "Port E pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL6_A::PORTE
    }
    #[doc = "Port F pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL6_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL6` writer - External Interrupt 6 Port Select"]
pub type EXTIPSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL6_A>;
impl<'a, REG> EXTIPSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6_A::PORTA)
    }
    #[doc = "Port B pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6_A::PORTB)
    }
    #[doc = "Port C pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6_A::PORTC)
    }
    #[doc = "Port D pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6_A::PORTD)
    }
    #[doc = "Port E pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6_A::PORTE)
    }
    #[doc = "Port F pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL6_A::PORTF)
    }
}
#[doc = "Field `EXTIPSEL7` reader - External Interrupt 7 Port Select"]
pub type EXTIPSEL7_R = crate::FieldReader<EXTIPSEL7_A>;
#[doc = "External Interrupt 7 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTIPSEL7_A {
    #[doc = "0: Port A pin 7 selected for external interrupt 7"]
    PORTA = 0,
    #[doc = "1: Port B pin 7 selected for external interrupt 7"]
    PORTB = 1,
    #[doc = "2: Port C pin 7 selected for external interrupt 7"]
    PORTC = 2,
    #[doc = "3: Port D pin 7 selected for external interrupt 7"]
    PORTD = 3,
    #[doc = "4: Port E pin 7 selected for external interrupt 7"]
    PORTE = 4,
    #[doc = "5: Port F pin 7 selected for external interrupt 7"]
    PORTF = 5,
}
impl From<EXTIPSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTIPSEL7_A {
    type Ux = u8;
}
impl EXTIPSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTIPSEL7_A> {
        match self.bits {
            0 => Some(EXTIPSEL7_A::PORTA),
            1 => Some(EXTIPSEL7_A::PORTB),
            2 => Some(EXTIPSEL7_A::PORTC),
            3 => Some(EXTIPSEL7_A::PORTD),
            4 => Some(EXTIPSEL7_A::PORTE),
            5 => Some(EXTIPSEL7_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Port A pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL7_A::PORTA
    }
    #[doc = "Port B pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL7_A::PORTB
    }
    #[doc = "Port C pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL7_A::PORTC
    }
    #[doc = "Port D pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL7_A::PORTD
    }
    #[doc = "Port E pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL7_A::PORTE
    }
    #[doc = "Port F pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL7_A::PORTF
    }
}
#[doc = "Field `EXTIPSEL7` writer - External Interrupt 7 Port Select"]
pub type EXTIPSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTIPSEL7_A>;
impl<'a, REG> EXTIPSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7_A::PORTA)
    }
    #[doc = "Port B pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7_A::PORTB)
    }
    #[doc = "Port C pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7_A::PORTC)
    }
    #[doc = "Port D pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7_A::PORTD)
    }
    #[doc = "Port E pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7_A::PORTE)
    }
    #[doc = "Port F pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTIPSEL7_A::PORTF)
    }
}
impl R {
    #[doc = "Bits 0:2 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> EXTIPSEL0_R {
        EXTIPSEL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> EXTIPSEL1_R {
        EXTIPSEL1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> EXTIPSEL2_R {
        EXTIPSEL2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> EXTIPSEL3_R {
        EXTIPSEL3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&self) -> EXTIPSEL4_R {
        EXTIPSEL4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&self) -> EXTIPSEL5_R {
        EXTIPSEL5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&self) -> EXTIPSEL6_R {
        EXTIPSEL6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&self) -> EXTIPSEL7_R {
        EXTIPSEL7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Interrupt 0 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel0(&mut self) -> EXTIPSEL0_W<EXTIPSELL_SPEC> {
        EXTIPSEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - External Interrupt 1 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel1(&mut self) -> EXTIPSEL1_W<EXTIPSELL_SPEC> {
        EXTIPSEL1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - External Interrupt 2 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel2(&mut self) -> EXTIPSEL2_W<EXTIPSELL_SPEC> {
        EXTIPSEL2_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - External Interrupt 3 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel3(&mut self) -> EXTIPSEL3_W<EXTIPSELL_SPEC> {
        EXTIPSEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - External Interrupt 4 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel4(&mut self) -> EXTIPSEL4_W<EXTIPSELL_SPEC> {
        EXTIPSEL4_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - External Interrupt 5 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel5(&mut self) -> EXTIPSEL5_W<EXTIPSELL_SPEC> {
        EXTIPSEL5_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - External Interrupt 6 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel6(&mut self) -> EXTIPSEL6_W<EXTIPSELL_SPEC> {
        EXTIPSEL6_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - External Interrupt 7 Port Select"]
    #[inline(always)]
    #[must_use]
    pub fn extipsel7(&mut self) -> EXTIPSEL7_W<EXTIPSELL_SPEC> {
        EXTIPSEL7_W::new(self, 28)
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
#[doc = "External Interrupt Port Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipsell::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipsell::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIPSELL_SPEC;
impl crate::RegisterSpec for EXTIPSELL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipsell::R`](R) reader structure"]
impl crate::Readable for EXTIPSELL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extipsell::W`](W) writer structure"]
impl crate::Writable for EXTIPSELL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIPSELL to value 0"]
impl crate::Resettable for EXTIPSELL_SPEC {
    const RESET_VALUE: u32 = 0;
}
