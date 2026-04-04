#[doc = "Register `TRACECTRL` reader"]
pub type R = crate::R<TracectrlSpec>;
#[doc = "Register `TRACECTRL` writer"]
pub type W = crate::W<TracectrlSpec>;
#[doc = "Field `TSTARTEN` reader - PRS TSTART Enable"]
pub type TstartenR = crate::BitReader;
#[doc = "Field `TSTARTEN` writer - PRS TSTART Enable"]
pub type TstartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MTB TSTART PRS select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstart {
    #[doc = "0: PRS ch 0 is controlling TSTART."]
    Prsch0 = 0,
    #[doc = "1: PRS ch 1 is controlling TSTART."]
    Prsch1 = 1,
    #[doc = "2: PRS ch 2 is controlling TSTART."]
    Prsch2 = 2,
    #[doc = "3: PRS ch 3 is controlling TSTART."]
    Prsch3 = 3,
    #[doc = "4: PRS ch 4 is controlling TSTART."]
    Prsch4 = 4,
    #[doc = "5: PRS ch 5 is controlling TSTART."]
    Prsch5 = 5,
}
impl From<Tstart> for u8 {
    #[inline(always)]
    fn from(variant: Tstart) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstart {
    type Ux = u8;
}
impl crate::IsEnum for Tstart {}
#[doc = "Field `TSTART` reader - MTB TSTART PRS select"]
pub type TstartR = crate::FieldReader<Tstart>;
impl TstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tstart> {
        match self.bits {
            0 => Some(Tstart::Prsch0),
            1 => Some(Tstart::Prsch1),
            2 => Some(Tstart::Prsch2),
            3 => Some(Tstart::Prsch3),
            4 => Some(Tstart::Prsch4),
            5 => Some(Tstart::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Tstart::Prsch0
    }
    #[doc = "PRS ch 1 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Tstart::Prsch1
    }
    #[doc = "PRS ch 2 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Tstart::Prsch2
    }
    #[doc = "PRS ch 3 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Tstart::Prsch3
    }
    #[doc = "PRS ch 4 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Tstart::Prsch4
    }
    #[doc = "PRS ch 5 is controlling TSTART."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Tstart::Prsch5
    }
}
#[doc = "Field `TSTART` writer - MTB TSTART PRS select"]
pub type TstartW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tstart>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Prsch0)
    }
    #[doc = "PRS ch 1 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Prsch1)
    }
    #[doc = "PRS ch 2 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Prsch2)
    }
    #[doc = "PRS ch 3 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Prsch3)
    }
    #[doc = "PRS ch 4 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Prsch4)
    }
    #[doc = "PRS ch 5 is controlling TSTART."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Prsch5)
    }
}
#[doc = "Field `TSTOPEN` reader - PRS TSTOP Enable"]
pub type TstopenR = crate::BitReader;
#[doc = "Field `TSTOPEN` writer - PRS TSTOP Enable"]
pub type TstopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MTB TSTOP PRS select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstop {
    #[doc = "0: PRS ch 0 is controlling TSTOP."]
    Prsch0 = 0,
    #[doc = "1: PRS ch 1 is controlling TSTOP."]
    Prsch1 = 1,
    #[doc = "2: PRS ch 2 is controlling TSTOP."]
    Prsch2 = 2,
    #[doc = "3: PRS ch 3 is controlling TSTOP."]
    Prsch3 = 3,
    #[doc = "4: PRS ch 4 is controlling TSTOP."]
    Prsch4 = 4,
    #[doc = "5: PRS ch 5 is controlling TSTOP."]
    Prsch5 = 5,
}
impl From<Tstop> for u8 {
    #[inline(always)]
    fn from(variant: Tstop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstop {
    type Ux = u8;
}
impl crate::IsEnum for Tstop {}
#[doc = "Field `TSTOP` reader - MTB TSTOP PRS select"]
pub type TstopR = crate::FieldReader<Tstop>;
impl TstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tstop> {
        match self.bits {
            0 => Some(Tstop::Prsch0),
            1 => Some(Tstop::Prsch1),
            2 => Some(Tstop::Prsch2),
            3 => Some(Tstop::Prsch3),
            4 => Some(Tstop::Prsch4),
            5 => Some(Tstop::Prsch5),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Tstop::Prsch0
    }
    #[doc = "PRS ch 1 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Tstop::Prsch1
    }
    #[doc = "PRS ch 2 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Tstop::Prsch2
    }
    #[doc = "PRS ch 3 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Tstop::Prsch3
    }
    #[doc = "PRS ch 4 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Tstop::Prsch4
    }
    #[doc = "PRS ch 5 is controlling TSTOP."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Tstop::Prsch5
    }
}
#[doc = "Field `TSTOP` writer - MTB TSTOP PRS select"]
pub type TstopW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tstop>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Prsch0)
    }
    #[doc = "PRS ch 1 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Prsch1)
    }
    #[doc = "PRS ch 2 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Prsch2)
    }
    #[doc = "PRS ch 3 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Prsch3)
    }
    #[doc = "PRS ch 4 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Prsch4)
    }
    #[doc = "PRS ch 5 is controlling TSTOP."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Prsch5)
    }
}
impl R {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    pub fn tstarten(&self) -> TstartenR {
        TstartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    pub fn tstopen(&self) -> TstopenR {
        TstopenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline(always)]
    pub fn tstop(&self) -> TstopR {
        TstopR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PRS TSTART Enable"]
    #[inline(always)]
    pub fn tstarten(&mut self) -> TstartenW<'_, TracectrlSpec> {
        TstartenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - MTB TSTART PRS select"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<'_, TracectrlSpec> {
        TstartW::new(self, 1)
    }
    #[doc = "Bit 8 - PRS TSTOP Enable"]
    #[inline(always)]
    pub fn tstopen(&mut self) -> TstopenW<'_, TracectrlSpec> {
        TstopenW::new(self, 8)
    }
    #[doc = "Bits 9:11 - MTB TSTOP PRS select"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TstopW<'_, TracectrlSpec> {
        TstopW::new(self, 9)
    }
}
#[doc = "MTB Trace Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tracectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tracectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TracectrlSpec;
impl crate::RegisterSpec for TracectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tracectrl::R`](R) reader structure"]
impl crate::Readable for TracectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tracectrl::W`](W) writer structure"]
impl crate::Writable for TracectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACECTRL to value 0"]
impl crate::Resettable for TracectrlSpec {}
