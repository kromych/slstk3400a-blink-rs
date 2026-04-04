#[doc = "Register `EXTIPSELH` reader"]
pub type R = crate::R<ExtipselhSpec>;
#[doc = "Register `EXTIPSELH` writer"]
pub type W = crate::W<ExtipselhSpec>;
#[doc = "External Interrupt 8 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel8 {
    #[doc = "0: Port A pin 8 selected for external interrupt 8"]
    Porta = 0,
    #[doc = "1: Port B pin 8 selected for external interrupt 8"]
    Portb = 1,
    #[doc = "2: Port C pin 8 selected for external interrupt 8"]
    Portc = 2,
    #[doc = "3: Port D pin 8 selected for external interrupt 8"]
    Portd = 3,
    #[doc = "4: Port E pin 8 selected for external interrupt 8"]
    Porte = 4,
    #[doc = "5: Port F pin 8 selected for external interrupt 8"]
    Portf = 5,
}
impl From<Extipsel8> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel8 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel8 {}
#[doc = "Field `EXTIPSEL8` reader - External Interrupt 8 Port Select"]
pub type Extipsel8R = crate::FieldReader<Extipsel8>;
impl Extipsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel8> {
        match self.bits {
            0 => Some(Extipsel8::Porta),
            1 => Some(Extipsel8::Portb),
            2 => Some(Extipsel8::Portc),
            3 => Some(Extipsel8::Portd),
            4 => Some(Extipsel8::Porte),
            5 => Some(Extipsel8::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel8::Porta
    }
    #[doc = "Port B pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel8::Portb
    }
    #[doc = "Port C pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel8::Portc
    }
    #[doc = "Port D pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel8::Portd
    }
    #[doc = "Port E pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel8::Porte
    }
    #[doc = "Port F pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel8::Portf
    }
}
#[doc = "Field `EXTIPSEL8` writer - External Interrupt 8 Port Select"]
pub type Extipsel8W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel8>;
impl<'a, REG> Extipsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel8::Porta)
    }
    #[doc = "Port B pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel8::Portb)
    }
    #[doc = "Port C pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel8::Portc)
    }
    #[doc = "Port D pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel8::Portd)
    }
    #[doc = "Port E pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel8::Porte)
    }
    #[doc = "Port F pin 8 selected for external interrupt 8"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel8::Portf)
    }
}
#[doc = "External Interrupt 9 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel9 {
    #[doc = "0: Port A pin 9 selected for external interrupt 9"]
    Porta = 0,
    #[doc = "1: Port B pin 9 selected for external interrupt 9"]
    Portb = 1,
    #[doc = "2: Port C pin 9 selected for external interrupt 9"]
    Portc = 2,
    #[doc = "3: Port D pin 9 selected for external interrupt 9"]
    Portd = 3,
    #[doc = "4: Port E pin 9 selected for external interrupt 9"]
    Porte = 4,
    #[doc = "5: Port F pin 9 selected for external interrupt 9"]
    Portf = 5,
}
impl From<Extipsel9> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel9 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel9 {}
#[doc = "Field `EXTIPSEL9` reader - External Interrupt 9 Port Select"]
pub type Extipsel9R = crate::FieldReader<Extipsel9>;
impl Extipsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel9> {
        match self.bits {
            0 => Some(Extipsel9::Porta),
            1 => Some(Extipsel9::Portb),
            2 => Some(Extipsel9::Portc),
            3 => Some(Extipsel9::Portd),
            4 => Some(Extipsel9::Porte),
            5 => Some(Extipsel9::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel9::Porta
    }
    #[doc = "Port B pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel9::Portb
    }
    #[doc = "Port C pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel9::Portc
    }
    #[doc = "Port D pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel9::Portd
    }
    #[doc = "Port E pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel9::Porte
    }
    #[doc = "Port F pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel9::Portf
    }
}
#[doc = "Field `EXTIPSEL9` writer - External Interrupt 9 Port Select"]
pub type Extipsel9W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel9>;
impl<'a, REG> Extipsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel9::Porta)
    }
    #[doc = "Port B pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel9::Portb)
    }
    #[doc = "Port C pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel9::Portc)
    }
    #[doc = "Port D pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel9::Portd)
    }
    #[doc = "Port E pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel9::Porte)
    }
    #[doc = "Port F pin 9 selected for external interrupt 9"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel9::Portf)
    }
}
#[doc = "External Interrupt 10 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel10 {
    #[doc = "0: Port A pin 10 selected for external interrupt 10"]
    Porta = 0,
    #[doc = "1: Port B pin 10 selected for external interrupt 10"]
    Portb = 1,
    #[doc = "2: Port C pin 10 selected for external interrupt 10"]
    Portc = 2,
    #[doc = "3: Port D pin 10 selected for external interrupt 10"]
    Portd = 3,
    #[doc = "4: Port E pin 10 selected for external interrupt 10"]
    Porte = 4,
    #[doc = "5: Port F pin 10 selected for external interrupt 10"]
    Portf = 5,
}
impl From<Extipsel10> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel10 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel10 {}
#[doc = "Field `EXTIPSEL10` reader - External Interrupt 10 Port Select"]
pub type Extipsel10R = crate::FieldReader<Extipsel10>;
impl Extipsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel10> {
        match self.bits {
            0 => Some(Extipsel10::Porta),
            1 => Some(Extipsel10::Portb),
            2 => Some(Extipsel10::Portc),
            3 => Some(Extipsel10::Portd),
            4 => Some(Extipsel10::Porte),
            5 => Some(Extipsel10::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel10::Porta
    }
    #[doc = "Port B pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel10::Portb
    }
    #[doc = "Port C pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel10::Portc
    }
    #[doc = "Port D pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel10::Portd
    }
    #[doc = "Port E pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel10::Porte
    }
    #[doc = "Port F pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel10::Portf
    }
}
#[doc = "Field `EXTIPSEL10` writer - External Interrupt 10 Port Select"]
pub type Extipsel10W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel10>;
impl<'a, REG> Extipsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel10::Porta)
    }
    #[doc = "Port B pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel10::Portb)
    }
    #[doc = "Port C pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel10::Portc)
    }
    #[doc = "Port D pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel10::Portd)
    }
    #[doc = "Port E pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel10::Porte)
    }
    #[doc = "Port F pin 10 selected for external interrupt 10"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel10::Portf)
    }
}
#[doc = "External Interrupt 11 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel11 {
    #[doc = "0: Port A pin 11 selected for external interrupt 11"]
    Porta = 0,
    #[doc = "1: Port B pin 11 selected for external interrupt 11"]
    Portb = 1,
    #[doc = "2: Port C pin 11 selected for external interrupt 11"]
    Portc = 2,
    #[doc = "3: Port D pin 11 selected for external interrupt 11"]
    Portd = 3,
    #[doc = "4: Port E pin 11 selected for external interrupt 11"]
    Porte = 4,
    #[doc = "5: Port F pin 11 selected for external interrupt 11"]
    Portf = 5,
}
impl From<Extipsel11> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel11 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel11 {}
#[doc = "Field `EXTIPSEL11` reader - External Interrupt 11 Port Select"]
pub type Extipsel11R = crate::FieldReader<Extipsel11>;
impl Extipsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel11> {
        match self.bits {
            0 => Some(Extipsel11::Porta),
            1 => Some(Extipsel11::Portb),
            2 => Some(Extipsel11::Portc),
            3 => Some(Extipsel11::Portd),
            4 => Some(Extipsel11::Porte),
            5 => Some(Extipsel11::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel11::Porta
    }
    #[doc = "Port B pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel11::Portb
    }
    #[doc = "Port C pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel11::Portc
    }
    #[doc = "Port D pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel11::Portd
    }
    #[doc = "Port E pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel11::Porte
    }
    #[doc = "Port F pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel11::Portf
    }
}
#[doc = "Field `EXTIPSEL11` writer - External Interrupt 11 Port Select"]
pub type Extipsel11W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel11>;
impl<'a, REG> Extipsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel11::Porta)
    }
    #[doc = "Port B pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel11::Portb)
    }
    #[doc = "Port C pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel11::Portc)
    }
    #[doc = "Port D pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel11::Portd)
    }
    #[doc = "Port E pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel11::Porte)
    }
    #[doc = "Port F pin 11 selected for external interrupt 11"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel11::Portf)
    }
}
#[doc = "External Interrupt 12 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel12 {
    #[doc = "0: Port A pin 12 selected for external interrupt 12"]
    Porta = 0,
    #[doc = "1: Port B pin 12 selected for external interrupt 12"]
    Portb = 1,
    #[doc = "2: Port C pin 12 selected for external interrupt 12"]
    Portc = 2,
    #[doc = "3: Port D pin 12 selected for external interrupt 12"]
    Portd = 3,
    #[doc = "4: Port E pin 12 selected for external interrupt 12"]
    Porte = 4,
    #[doc = "5: Port F pin 12 selected for external interrupt 12"]
    Portf = 5,
}
impl From<Extipsel12> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel12 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel12 {}
#[doc = "Field `EXTIPSEL12` reader - External Interrupt 12 Port Select"]
pub type Extipsel12R = crate::FieldReader<Extipsel12>;
impl Extipsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel12> {
        match self.bits {
            0 => Some(Extipsel12::Porta),
            1 => Some(Extipsel12::Portb),
            2 => Some(Extipsel12::Portc),
            3 => Some(Extipsel12::Portd),
            4 => Some(Extipsel12::Porte),
            5 => Some(Extipsel12::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel12::Porta
    }
    #[doc = "Port B pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel12::Portb
    }
    #[doc = "Port C pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel12::Portc
    }
    #[doc = "Port D pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel12::Portd
    }
    #[doc = "Port E pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel12::Porte
    }
    #[doc = "Port F pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel12::Portf
    }
}
#[doc = "Field `EXTIPSEL12` writer - External Interrupt 12 Port Select"]
pub type Extipsel12W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel12>;
impl<'a, REG> Extipsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel12::Porta)
    }
    #[doc = "Port B pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel12::Portb)
    }
    #[doc = "Port C pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel12::Portc)
    }
    #[doc = "Port D pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel12::Portd)
    }
    #[doc = "Port E pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel12::Porte)
    }
    #[doc = "Port F pin 12 selected for external interrupt 12"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel12::Portf)
    }
}
#[doc = "External Interrupt 13 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel13 {
    #[doc = "0: Port A pin 13 selected for external interrupt 13"]
    Porta = 0,
    #[doc = "1: Port B pin 13 selected for external interrupt 13"]
    Portb = 1,
    #[doc = "2: Port C pin 13 selected for external interrupt 13"]
    Portc = 2,
    #[doc = "3: Port D pin 13 selected for external interrupt 13"]
    Portd = 3,
    #[doc = "4: Port E pin 13 selected for external interrupt 13"]
    Porte = 4,
    #[doc = "5: Port F pin 13 selected for external interrupt 13"]
    Portf = 5,
}
impl From<Extipsel13> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel13 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel13 {}
#[doc = "Field `EXTIPSEL13` reader - External Interrupt 13 Port Select"]
pub type Extipsel13R = crate::FieldReader<Extipsel13>;
impl Extipsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel13> {
        match self.bits {
            0 => Some(Extipsel13::Porta),
            1 => Some(Extipsel13::Portb),
            2 => Some(Extipsel13::Portc),
            3 => Some(Extipsel13::Portd),
            4 => Some(Extipsel13::Porte),
            5 => Some(Extipsel13::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel13::Porta
    }
    #[doc = "Port B pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel13::Portb
    }
    #[doc = "Port C pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel13::Portc
    }
    #[doc = "Port D pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel13::Portd
    }
    #[doc = "Port E pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel13::Porte
    }
    #[doc = "Port F pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel13::Portf
    }
}
#[doc = "Field `EXTIPSEL13` writer - External Interrupt 13 Port Select"]
pub type Extipsel13W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel13>;
impl<'a, REG> Extipsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel13::Porta)
    }
    #[doc = "Port B pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel13::Portb)
    }
    #[doc = "Port C pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel13::Portc)
    }
    #[doc = "Port D pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel13::Portd)
    }
    #[doc = "Port E pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel13::Porte)
    }
    #[doc = "Port F pin 13 selected for external interrupt 13"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel13::Portf)
    }
}
#[doc = "External Interrupt 14 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel14 {
    #[doc = "0: Port A pin 14 selected for external interrupt 14"]
    Porta = 0,
    #[doc = "1: Port B pin 14 selected for external interrupt 14"]
    Portb = 1,
    #[doc = "2: Port C pin 14 selected for external interrupt 14"]
    Portc = 2,
    #[doc = "3: Port D pin 14 selected for external interrupt 14"]
    Portd = 3,
    #[doc = "4: Port E pin 14 selected for external interrupt 14"]
    Porte = 4,
    #[doc = "5: Port F pin 14 selected for external interrupt 14"]
    Portf = 5,
}
impl From<Extipsel14> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel14 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel14 {}
#[doc = "Field `EXTIPSEL14` reader - External Interrupt 14 Port Select"]
pub type Extipsel14R = crate::FieldReader<Extipsel14>;
impl Extipsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel14> {
        match self.bits {
            0 => Some(Extipsel14::Porta),
            1 => Some(Extipsel14::Portb),
            2 => Some(Extipsel14::Portc),
            3 => Some(Extipsel14::Portd),
            4 => Some(Extipsel14::Porte),
            5 => Some(Extipsel14::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel14::Porta
    }
    #[doc = "Port B pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel14::Portb
    }
    #[doc = "Port C pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel14::Portc
    }
    #[doc = "Port D pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel14::Portd
    }
    #[doc = "Port E pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel14::Porte
    }
    #[doc = "Port F pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel14::Portf
    }
}
#[doc = "Field `EXTIPSEL14` writer - External Interrupt 14 Port Select"]
pub type Extipsel14W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel14>;
impl<'a, REG> Extipsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel14::Porta)
    }
    #[doc = "Port B pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel14::Portb)
    }
    #[doc = "Port C pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel14::Portc)
    }
    #[doc = "Port D pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel14::Portd)
    }
    #[doc = "Port E pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel14::Porte)
    }
    #[doc = "Port F pin 14 selected for external interrupt 14"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel14::Portf)
    }
}
#[doc = "External Interrupt 15 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel15 {
    #[doc = "0: Port A pin 15 selected for external interrupt 15"]
    Porta = 0,
    #[doc = "1: Port B pin 15 selected for external interrupt 15"]
    Portb = 1,
    #[doc = "2: Port C pin 15 selected for external interrupt 15"]
    Portc = 2,
    #[doc = "3: Port D pin 15 selected for external interrupt 15"]
    Portd = 3,
    #[doc = "4: Port E pin 15 selected for external interrupt 15"]
    Porte = 4,
    #[doc = "5: Port F pin 15 selected for external interrupt 15"]
    Portf = 5,
}
impl From<Extipsel15> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel15 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel15 {}
#[doc = "Field `EXTIPSEL15` reader - External Interrupt 15 Port Select"]
pub type Extipsel15R = crate::FieldReader<Extipsel15>;
impl Extipsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extipsel15> {
        match self.bits {
            0 => Some(Extipsel15::Porta),
            1 => Some(Extipsel15::Portb),
            2 => Some(Extipsel15::Portc),
            3 => Some(Extipsel15::Portd),
            4 => Some(Extipsel15::Porte),
            5 => Some(Extipsel15::Portf),
            _ => None,
        }
    }
    #[doc = "Port A pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel15::Porta
    }
    #[doc = "Port B pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel15::Portb
    }
    #[doc = "Port C pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel15::Portc
    }
    #[doc = "Port D pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel15::Portd
    }
    #[doc = "Port E pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == Extipsel15::Porte
    }
    #[doc = "Port F pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == Extipsel15::Portf
    }
}
#[doc = "Field `EXTIPSEL15` writer - External Interrupt 15 Port Select"]
pub type Extipsel15W<'a, REG> = crate::FieldWriter<'a, REG, 3, Extipsel15>;
impl<'a, REG> Extipsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel15::Porta)
    }
    #[doc = "Port B pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel15::Portb)
    }
    #[doc = "Port C pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel15::Portc)
    }
    #[doc = "Port D pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel15::Portd)
    }
    #[doc = "Port E pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel15::Porte)
    }
    #[doc = "Port F pin 15 selected for external interrupt 15"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel15::Portf)
    }
}
impl R {
    #[doc = "Bits 0:2 - External Interrupt 8 Port Select"]
    #[inline(always)]
    pub fn extipsel8(&self) -> Extipsel8R {
        Extipsel8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - External Interrupt 9 Port Select"]
    #[inline(always)]
    pub fn extipsel9(&self) -> Extipsel9R {
        Extipsel9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - External Interrupt 10 Port Select"]
    #[inline(always)]
    pub fn extipsel10(&self) -> Extipsel10R {
        Extipsel10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - External Interrupt 11 Port Select"]
    #[inline(always)]
    pub fn extipsel11(&self) -> Extipsel11R {
        Extipsel11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - External Interrupt 12 Port Select"]
    #[inline(always)]
    pub fn extipsel12(&self) -> Extipsel12R {
        Extipsel12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - External Interrupt 13 Port Select"]
    #[inline(always)]
    pub fn extipsel13(&self) -> Extipsel13R {
        Extipsel13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - External Interrupt 14 Port Select"]
    #[inline(always)]
    pub fn extipsel14(&self) -> Extipsel14R {
        Extipsel14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - External Interrupt 15 Port Select"]
    #[inline(always)]
    pub fn extipsel15(&self) -> Extipsel15R {
        Extipsel15R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Interrupt 8 Port Select"]
    #[inline(always)]
    pub fn extipsel8(&mut self) -> Extipsel8W<'_, ExtipselhSpec> {
        Extipsel8W::new(self, 0)
    }
    #[doc = "Bits 4:6 - External Interrupt 9 Port Select"]
    #[inline(always)]
    pub fn extipsel9(&mut self) -> Extipsel9W<'_, ExtipselhSpec> {
        Extipsel9W::new(self, 4)
    }
    #[doc = "Bits 8:10 - External Interrupt 10 Port Select"]
    #[inline(always)]
    pub fn extipsel10(&mut self) -> Extipsel10W<'_, ExtipselhSpec> {
        Extipsel10W::new(self, 8)
    }
    #[doc = "Bits 12:14 - External Interrupt 11 Port Select"]
    #[inline(always)]
    pub fn extipsel11(&mut self) -> Extipsel11W<'_, ExtipselhSpec> {
        Extipsel11W::new(self, 12)
    }
    #[doc = "Bits 16:18 - External Interrupt 12 Port Select"]
    #[inline(always)]
    pub fn extipsel12(&mut self) -> Extipsel12W<'_, ExtipselhSpec> {
        Extipsel12W::new(self, 16)
    }
    #[doc = "Bits 20:22 - External Interrupt 13 Port Select"]
    #[inline(always)]
    pub fn extipsel13(&mut self) -> Extipsel13W<'_, ExtipselhSpec> {
        Extipsel13W::new(self, 20)
    }
    #[doc = "Bits 24:26 - External Interrupt 14 Port Select"]
    #[inline(always)]
    pub fn extipsel14(&mut self) -> Extipsel14W<'_, ExtipselhSpec> {
        Extipsel14W::new(self, 24)
    }
    #[doc = "Bits 28:30 - External Interrupt 15 Port Select"]
    #[inline(always)]
    pub fn extipsel15(&mut self) -> Extipsel15W<'_, ExtipselhSpec> {
        Extipsel15W::new(self, 28)
    }
}
#[doc = "External Interrupt Port Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtipselhSpec;
impl crate::RegisterSpec for ExtipselhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipselh::R`](R) reader structure"]
impl crate::Readable for ExtipselhSpec {}
#[doc = "`write(|w| ..)` method takes [`extipselh::W`](W) writer structure"]
impl crate::Writable for ExtipselhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIPSELH to value 0"]
impl crate::Resettable for ExtipselhSpec {}
