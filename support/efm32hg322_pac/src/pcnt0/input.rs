#[doc = "Register `INPUT` reader"]
pub type R = crate::R<InputSpec>;
#[doc = "Register `INPUT` writer"]
pub type W = crate::W<InputSpec>;
#[doc = "S0IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0prssel {
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
impl From<S0prssel> for u8 {
    #[inline(always)]
    fn from(variant: S0prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0prssel {
    type Ux = u8;
}
impl crate::IsEnum for S0prssel {}
#[doc = "Field `S0PRSSEL` reader - S0IN PRS Channel Select"]
pub type S0prsselR = crate::FieldReader<S0prssel>;
impl S0prsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0prssel> {
        match self.bits {
            0 => Some(S0prssel::Prsch0),
            1 => Some(S0prssel::Prsch1),
            2 => Some(S0prssel::Prsch2),
            3 => Some(S0prssel::Prsch3),
            4 => Some(S0prssel::Prsch4),
            5 => Some(S0prssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S0prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S0prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S0prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S0prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S0prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S0prssel::Prsch5
    }
}
#[doc = "Field `S0PRSSEL` writer - S0IN PRS Channel Select"]
pub type S0prsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, S0prssel>;
impl<'a, REG> S0prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch5)
    }
}
#[doc = "Field `S0PRSEN` reader - S0IN PRS Enable"]
pub type S0prsenR = crate::BitReader;
#[doc = "Field `S0PRSEN` writer - S0IN PRS Enable"]
pub type S0prsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "S1IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1prssel {
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
impl From<S1prssel> for u8 {
    #[inline(always)]
    fn from(variant: S1prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1prssel {
    type Ux = u8;
}
impl crate::IsEnum for S1prssel {}
#[doc = "Field `S1PRSSEL` reader - S1IN PRS Channel Select"]
pub type S1prsselR = crate::FieldReader<S1prssel>;
impl S1prsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1prssel> {
        match self.bits {
            0 => Some(S1prssel::Prsch0),
            1 => Some(S1prssel::Prsch1),
            2 => Some(S1prssel::Prsch2),
            3 => Some(S1prssel::Prsch3),
            4 => Some(S1prssel::Prsch4),
            5 => Some(S1prssel::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S1prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S1prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S1prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S1prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S1prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S1prssel::Prsch5
    }
}
#[doc = "Field `S1PRSSEL` writer - S1IN PRS Channel Select"]
pub type S1prsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, S1prssel>;
impl<'a, REG> S1prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch5)
    }
}
#[doc = "Field `S1PRSEN` reader - S1IN PRS Enable"]
pub type S1prsenR = crate::BitReader;
#[doc = "Field `S1PRSEN` writer - S1IN PRS Enable"]
pub type S1prsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&self) -> S0prsselR {
        S0prsselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0prsenR {
        S0prsenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:8 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&self) -> S1prsselR {
        S1prsselR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 10 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1prsenR {
        S1prsenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&mut self) -> S0prsselW<'_, InputSpec> {
        S0prsselW::new(self, 0)
    }
    #[doc = "Bit 4 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&mut self) -> S0prsenW<'_, InputSpec> {
        S0prsenW::new(self, 4)
    }
    #[doc = "Bits 6:8 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&mut self) -> S1prsselW<'_, InputSpec> {
        S1prsselW::new(self, 6)
    }
    #[doc = "Bit 10 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&mut self) -> S1prsenW<'_, InputSpec> {
        S1prsenW::new(self, 10)
    }
}
#[doc = "PCNT Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputSpec;
impl crate::RegisterSpec for InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for InputSpec {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for InputSpec {}
