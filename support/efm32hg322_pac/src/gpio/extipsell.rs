#[doc = "Register `EXTIPSELL` reader"]
pub type R = crate::R<ExtipsellSpec>;
#[doc = "Register `EXTIPSELL` writer"]
pub type W = crate::W<ExtipsellSpec>;
#[doc = "External Interrupt 0 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel0 {
    #[doc = "0: Port A pin 0 selected for external interrupt 0"]
    Porta = 0,
    #[doc = "1: Port B pin 0 selected for external interrupt 0"]
    Portb = 1,
    #[doc = "2: Port C pin 0 selected for external interrupt 0"]
    Portc = 2,
    #[doc = "3: Port D pin 0 selected for external interrupt 0"]
    Portd = 3,
    #[doc = "4: Port E pin 0 selected for external interrupt 0"]
    Porte = 4,
    #[doc = "5: Port F pin 0 selected for external interrupt 0"]
    Portf = 5,
}
impl From<Extipsel0> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel0 {}
#[doc = "Field `EXTIPSEL0` reader - External Interrupt 0 Port Select"]
pub type Extipsel0R = crate::FieldReader<Extipsel0>;
impl Extipsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel0> {
        match self.bits {
            0 => Some(Extipsel0::Porta),
            1 => Some(Extipsel0::Portb),
            2 => Some(Extipsel0::Portc),
            3 => Some(Extipsel0::Portd),
            4 => Some(Extipsel0::Porte),
            5 => Some(Extipsel0::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel0::Porta
    }
    #[doc = "Port B pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel0::Portb
    }
    #[doc = "Port C pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel0::Portc
    }
    #[doc = "Port D pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel0::Portd
    }
    #[doc = "Port E pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel0::Porte
    }
    #[doc = "Port F pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel0::Portf
    }
}
#[doc = "Field `EXTIPSEL0` writer - External Interrupt 0 Port Select"]
pub type Extipsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel0>;
impl<'a, REG> Extipsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Porta)
    }
    #[doc = "Port B pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Portb)
    }
    #[doc = "Port C pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Portc)
    }
    #[doc = "Port D pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Portd)
    }
    #[doc = "Port E pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Porte)
    }
    #[doc = "Port F pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Portf)
    }
}
#[doc = "External Interrupt 1 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel1 {
    #[doc = "0: Port A pin 1 selected for external interrupt 1"]
    Porta = 0,
    #[doc = "1: Port B pin 1 selected for external interrupt 1"]
    Portb = 1,
    #[doc = "2: Port C pin 1 selected for external interrupt 1"]
    Portc = 2,
    #[doc = "3: Port D pin 1 selected for external interrupt 1"]
    Portd = 3,
    #[doc = "4: Port E pin 1 selected for external interrupt 1"]
    Porte = 4,
    #[doc = "5: Port F pin 1 selected for external interrupt 1"]
    Portf = 5,
}
impl From<Extipsel1> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel1 {}
#[doc = "Field `EXTIPSEL1` reader - External Interrupt 1 Port Select"]
pub type Extipsel1R = crate::FieldReader<Extipsel1>;
impl Extipsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel1> {
        match self.bits {
            0 => Some(Extipsel1::Porta),
            1 => Some(Extipsel1::Portb),
            2 => Some(Extipsel1::Portc),
            3 => Some(Extipsel1::Portd),
            4 => Some(Extipsel1::Porte),
            5 => Some(Extipsel1::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel1::Porta
    }
    #[doc = "Port B pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel1::Portb
    }
    #[doc = "Port C pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel1::Portc
    }
    #[doc = "Port D pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel1::Portd
    }
    #[doc = "Port E pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel1::Porte
    }
    #[doc = "Port F pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel1::Portf
    }
}
#[doc = "Field `EXTIPSEL1` writer - External Interrupt 1 Port Select"]
pub type Extipsel1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel1>;
impl<'a, REG> Extipsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Porta)
    }
    #[doc = "Port B pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Portb)
    }
    #[doc = "Port C pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Portc)
    }
    #[doc = "Port D pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Portd)
    }
    #[doc = "Port E pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Porte)
    }
    #[doc = "Port F pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Portf)
    }
}
#[doc = "External Interrupt 2 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel2 {
    #[doc = "0: Port A pin 2 selected for external interrupt 2"]
    Porta = 0,
    #[doc = "1: Port B pin 2 selected for external interrupt 2"]
    Portb = 1,
    #[doc = "2: Port C pin 2 selected for external interrupt 2"]
    Portc = 2,
    #[doc = "3: Port D pin 2 selected for external interrupt 2"]
    Portd = 3,
    #[doc = "4: Port E pin 2 selected for external interrupt 2"]
    Porte = 4,
    #[doc = "5: Port F pin 2 selected for external interrupt 2"]
    Portf = 5,
}
impl From<Extipsel2> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel2 {}
#[doc = "Field `EXTIPSEL2` reader - External Interrupt 2 Port Select"]
pub type Extipsel2R = crate::FieldReader<Extipsel2>;
impl Extipsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel2> {
        match self.bits {
            0 => Some(Extipsel2::Porta),
            1 => Some(Extipsel2::Portb),
            2 => Some(Extipsel2::Portc),
            3 => Some(Extipsel2::Portd),
            4 => Some(Extipsel2::Porte),
            5 => Some(Extipsel2::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel2::Porta
    }
    #[doc = "Port B pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel2::Portb
    }
    #[doc = "Port C pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel2::Portc
    }
    #[doc = "Port D pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel2::Portd
    }
    #[doc = "Port E pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel2::Porte
    }
    #[doc = "Port F pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel2::Portf
    }
}
#[doc = "Field `EXTIPSEL2` writer - External Interrupt 2 Port Select"]
pub type Extipsel2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel2>;
impl<'a, REG> Extipsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Porta)
    }
    #[doc = "Port B pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Portb)
    }
    #[doc = "Port C pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Portc)
    }
    #[doc = "Port D pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Portd)
    }
    #[doc = "Port E pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Porte)
    }
    #[doc = "Port F pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Portf)
    }
}
#[doc = "External Interrupt 3 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel3 {
    #[doc = "0: Port A pin 3 selected for external interrupt 3"]
    Porta = 0,
    #[doc = "1: Port B pin 3 selected for external interrupt 3"]
    Portb = 1,
    #[doc = "2: Port C pin 3 selected for external interrupt 3"]
    Portc = 2,
    #[doc = "3: Port D pin 3 selected for external interrupt 3"]
    Portd = 3,
    #[doc = "4: Port E pin 3 selected for external interrupt 3"]
    Porte = 4,
    #[doc = "5: Port F pin 3 selected for external interrupt 3"]
    Portf = 5,
}
impl From<Extipsel3> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel3 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel3 {}
#[doc = "Field `EXTIPSEL3` reader - External Interrupt 3 Port Select"]
pub type Extipsel3R = crate::FieldReader<Extipsel3>;
impl Extipsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel3> {
        match self.bits {
            0 => Some(Extipsel3::Porta),
            1 => Some(Extipsel3::Portb),
            2 => Some(Extipsel3::Portc),
            3 => Some(Extipsel3::Portd),
            4 => Some(Extipsel3::Porte),
            5 => Some(Extipsel3::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel3::Porta
    }
    #[doc = "Port B pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel3::Portb
    }
    #[doc = "Port C pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel3::Portc
    }
    #[doc = "Port D pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel3::Portd
    }
    #[doc = "Port E pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel3::Porte
    }
    #[doc = "Port F pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel3::Portf
    }
}
#[doc = "Field `EXTIPSEL3` writer - External Interrupt 3 Port Select"]
pub type Extipsel3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel3>;
impl<'a, REG> Extipsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Porta)
    }
    #[doc = "Port B pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Portb)
    }
    #[doc = "Port C pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Portc)
    }
    #[doc = "Port D pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Portd)
    }
    #[doc = "Port E pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Porte)
    }
    #[doc = "Port F pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Portf)
    }
}
#[doc = "External Interrupt 4 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel4 {
    #[doc = "0: Port A pin 4 selected for external interrupt 4"]
    Porta = 0,
    #[doc = "1: Port B pin 4 selected for external interrupt 4"]
    Portb = 1,
    #[doc = "2: Port C pin 4 selected for external interrupt 4"]
    Portc = 2,
    #[doc = "3: Port D pin 4 selected for external interrupt 4"]
    Portd = 3,
    #[doc = "4: Port E pin 4 selected for external interrupt 4"]
    Porte = 4,
    #[doc = "5: Port F pin 4 selected for external interrupt 4"]
    Portf = 5,
}
impl From<Extipsel4> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel4 {}
#[doc = "Field `EXTIPSEL4` reader - External Interrupt 4 Port Select"]
pub type Extipsel4R = crate::FieldReader<Extipsel4>;
impl Extipsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel4> {
        match self.bits {
            0 => Some(Extipsel4::Porta),
            1 => Some(Extipsel4::Portb),
            2 => Some(Extipsel4::Portc),
            3 => Some(Extipsel4::Portd),
            4 => Some(Extipsel4::Porte),
            5 => Some(Extipsel4::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel4::Porta
    }
    #[doc = "Port B pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel4::Portb
    }
    #[doc = "Port C pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel4::Portc
    }
    #[doc = "Port D pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel4::Portd
    }
    #[doc = "Port E pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel4::Porte
    }
    #[doc = "Port F pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel4::Portf
    }
}
#[doc = "Field `EXTIPSEL4` writer - External Interrupt 4 Port Select"]
pub type Extipsel4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel4>;
impl<'a, REG> Extipsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel4::Porta)
    }
    #[doc = "Port B pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel4::Portb)
    }
    #[doc = "Port C pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel4::Portc)
    }
    #[doc = "Port D pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel4::Portd)
    }
    #[doc = "Port E pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel4::Porte)
    }
    #[doc = "Port F pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel4::Portf)
    }
}
#[doc = "External Interrupt 5 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel5 {
    #[doc = "0: Port A pin 5 selected for external interrupt 5"]
    Porta = 0,
    #[doc = "1: Port B pin 5 selected for external interrupt 5"]
    Portb = 1,
    #[doc = "2: Port C pin 5 selected for external interrupt 5"]
    Portc = 2,
    #[doc = "3: Port D pin 5 selected for external interrupt 5"]
    Portd = 3,
    #[doc = "4: Port E pin 5 selected for external interrupt 5"]
    Porte = 4,
    #[doc = "5: Port F pin 5 selected for external interrupt 5"]
    Portf = 5,
}
impl From<Extipsel5> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel5 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel5 {}
#[doc = "Field `EXTIPSEL5` reader - External Interrupt 5 Port Select"]
pub type Extipsel5R = crate::FieldReader<Extipsel5>;
impl Extipsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel5> {
        match self.bits {
            0 => Some(Extipsel5::Porta),
            1 => Some(Extipsel5::Portb),
            2 => Some(Extipsel5::Portc),
            3 => Some(Extipsel5::Portd),
            4 => Some(Extipsel5::Porte),
            5 => Some(Extipsel5::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel5::Porta
    }
    #[doc = "Port B pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel5::Portb
    }
    #[doc = "Port C pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel5::Portc
    }
    #[doc = "Port D pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel5::Portd
    }
    #[doc = "Port E pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel5::Porte
    }
    #[doc = "Port F pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel5::Portf
    }
}
#[doc = "Field `EXTIPSEL5` writer - External Interrupt 5 Port Select"]
pub type Extipsel5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel5>;
impl<'a, REG> Extipsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel5::Porta)
    }
    #[doc = "Port B pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel5::Portb)
    }
    #[doc = "Port C pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel5::Portc)
    }
    #[doc = "Port D pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel5::Portd)
    }
    #[doc = "Port E pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel5::Porte)
    }
    #[doc = "Port F pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel5::Portf)
    }
}
#[doc = "External Interrupt 6 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel6 {
    #[doc = "0: Port A pin 6 selected for external interrupt 6"]
    Porta = 0,
    #[doc = "1: Port B pin 6 selected for external interrupt 6"]
    Portb = 1,
    #[doc = "2: Port C pin 6 selected for external interrupt 6"]
    Portc = 2,
    #[doc = "3: Port D pin 6 selected for external interrupt 6"]
    Portd = 3,
    #[doc = "4: Port E pin 6 selected for external interrupt 6"]
    Porte = 4,
    #[doc = "5: Port F pin 6 selected for external interrupt 6"]
    Portf = 5,
}
impl From<Extipsel6> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel6 {}
#[doc = "Field `EXTIPSEL6` reader - External Interrupt 6 Port Select"]
pub type Extipsel6R = crate::FieldReader<Extipsel6>;
impl Extipsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel6> {
        match self.bits {
            0 => Some(Extipsel6::Porta),
            1 => Some(Extipsel6::Portb),
            2 => Some(Extipsel6::Portc),
            3 => Some(Extipsel6::Portd),
            4 => Some(Extipsel6::Porte),
            5 => Some(Extipsel6::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel6::Porta
    }
    #[doc = "Port B pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel6::Portb
    }
    #[doc = "Port C pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel6::Portc
    }
    #[doc = "Port D pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel6::Portd
    }
    #[doc = "Port E pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel6::Porte
    }
    #[doc = "Port F pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel6::Portf
    }
}
#[doc = "Field `EXTIPSEL6` writer - External Interrupt 6 Port Select"]
pub type Extipsel6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel6>;
impl<'a, REG> Extipsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel6::Porta)
    }
    #[doc = "Port B pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel6::Portb)
    }
    #[doc = "Port C pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel6::Portc)
    }
    #[doc = "Port D pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel6::Portd)
    }
    #[doc = "Port E pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel6::Porte)
    }
    #[doc = "Port F pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel6::Portf)
    }
}
#[doc = "External Interrupt 7 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel7 {
    #[doc = "0: Port A pin 7 selected for external interrupt 7"]
    Porta = 0,
    #[doc = "1: Port B pin 7 selected for external interrupt 7"]
    Portb = 1,
    #[doc = "2: Port C pin 7 selected for external interrupt 7"]
    Portc = 2,
    #[doc = "3: Port D pin 7 selected for external interrupt 7"]
    Portd = 3,
    #[doc = "4: Port E pin 7 selected for external interrupt 7"]
    Porte = 4,
    #[doc = "5: Port F pin 7 selected for external interrupt 7"]
    Portf = 5,
}
impl From<Extipsel7> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel7 {}
#[doc = "Field `EXTIPSEL7` reader - External Interrupt 7 Port Select"]
pub type Extipsel7R = crate::FieldReader<Extipsel7>;
impl Extipsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel7> {
        match self.bits {
            0 => Some(Extipsel7::Porta),
            1 => Some(Extipsel7::Portb),
            2 => Some(Extipsel7::Portc),
            3 => Some(Extipsel7::Portd),
            4 => Some(Extipsel7::Porte),
            5 => Some(Extipsel7::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel7::Porta
    }
    #[doc = "Port B pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel7::Portb
    }
    #[doc = "Port C pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel7::Portc
    }
    #[doc = "Port D pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel7::Portd
    }
    #[doc = "Port E pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel7::Porte
    }
    #[doc = "Port F pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel7::Portf
    }
}
#[doc = "Field `EXTIPSEL7` writer - External Interrupt 7 Port Select"]
pub type Extipsel7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel7>;
impl<'a, REG> Extipsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel7::Porta)
    }
    #[doc = "Port B pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel7::Portb)
    }
    #[doc = "Port C pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel7::Portc)
    }
    #[doc = "Port D pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel7::Portd)
    }
    #[doc = "Port E pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel7::Porte)
    }
    #[doc = "Port F pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel7::Portf)
    }
}
impl R {
    #[doc = "Bits 0:2 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> Extipsel0R {
        Extipsel0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> Extipsel1R {
        Extipsel1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> Extipsel2R {
        Extipsel2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> Extipsel3R {
        Extipsel3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&self) -> Extipsel4R {
        Extipsel4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&self) -> Extipsel5R {
        Extipsel5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&self) -> Extipsel6R {
        Extipsel6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&self) -> Extipsel7R {
        Extipsel7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&mut self) -> Extipsel0W<'_, ExtipsellSpec> {
        Extipsel0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&mut self) -> Extipsel1W<'_, ExtipsellSpec> {
        Extipsel1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&mut self) -> Extipsel2W<'_, ExtipsellSpec> {
        Extipsel2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&mut self) -> Extipsel3W<'_, ExtipsellSpec> {
        Extipsel3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&mut self) -> Extipsel4W<'_, ExtipsellSpec> {
        Extipsel4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&mut self) -> Extipsel5W<'_, ExtipsellSpec> {
        Extipsel5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&mut self) -> Extipsel6W<'_, ExtipsellSpec> {
        Extipsel6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&mut self) -> Extipsel7W<'_, ExtipsellSpec> {
        Extipsel7W::new(self, 28)
    }
}
#[doc = "External Interrupt Port Select Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipsell::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipsell::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtipsellSpec;
impl crate::RegisterSpec for ExtipsellSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipsell::R`](R) reader structure"]
impl crate::Readable for ExtipsellSpec {}
#[doc = "`write(|w| ..)` method takes [`extipsell::W`](W) writer structure"]
impl crate::Writable for ExtipsellSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIPSELL to value 0"]
impl crate::Resettable for ExtipsellSpec {}
