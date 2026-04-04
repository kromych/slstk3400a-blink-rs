#[doc = "Register `CURPROG` reader"]
pub type R = crate::R<CurprogSpec>;
#[doc = "Register `CURPROG` writer"]
pub type W = crate::W<CurprogSpec>;
#[doc = "Current Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rangesel {
    #[doc = "0: Current range set to 0 - 1.6 uA."]
    Range0 = 0,
    #[doc = "1: Current range set to 1.6 - 4.7 uA."]
    Range1 = 1,
    #[doc = "2: Current range set to 0.5 - 16 uA."]
    Range2 = 2,
    #[doc = "3: Current range set to 2 - 64 uA."]
    Range3 = 3,
}
impl From<Rangesel> for u8 {
    #[inline(always)]
    fn from(variant: Rangesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rangesel {
    type Ux = u8;
}
impl crate::IsEnum for Rangesel {}
#[doc = "Field `RANGESEL` reader - Current Range Select"]
pub type RangeselR = crate::FieldReader<Rangesel>;
impl RangeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rangesel {
        match self.bits {
            0 => Rangesel::Range0,
            1 => Rangesel::Range1,
            2 => Rangesel::Range2,
            3 => Rangesel::Range3,
            _ => unreachable!(),
        }
    }
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == Rangesel::Range0
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == Rangesel::Range1
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == Rangesel::Range2
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == Rangesel::Range3
    }
}
#[doc = "Field `RANGESEL` writer - Current Range Select"]
pub type RangeselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rangesel, crate::Safe>;
impl<'a, REG> RangeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn range0(self) -> &'a mut crate::W<REG> {
        self.variant(Rangesel::Range0)
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(Rangesel::Range1)
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(Rangesel::Range2)
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(Rangesel::Range3)
    }
}
#[doc = "Field `STEPSEL` reader - Current Step Size Select"]
pub type StepselR = crate::FieldReader;
#[doc = "Field `STEPSEL` writer - Current Step Size Select"]
pub type StepselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&self) -> RangeselR {
        RangeselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&self) -> StepselR {
        StepselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&mut self) -> RangeselW<'_, CurprogSpec> {
        RangeselW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&mut self) -> StepselW<'_, CurprogSpec> {
        StepselW::new(self, 8)
    }
}
#[doc = "Current Programming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`curprog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`curprog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurprogSpec;
impl crate::RegisterSpec for CurprogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curprog::R`](R) reader structure"]
impl crate::Readable for CurprogSpec {}
#[doc = "`write(|w| ..)` method takes [`curprog::W`](W) writer structure"]
impl crate::Writable for CurprogSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CURPROG to value 0"]
impl crate::Resettable for CurprogSpec {}
