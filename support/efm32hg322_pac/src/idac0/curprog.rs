#[doc = "Register `CURPROG` reader"]
pub type R = crate::R<CURPROG_SPEC>;
#[doc = "Register `CURPROG` writer"]
pub type W = crate::W<CURPROG_SPEC>;
#[doc = "Field `RANGESEL` reader - Current Range Select"]
pub type RANGESEL_R = crate::FieldReader<RANGESEL_A>;
#[doc = "Current Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGESEL_A {
    #[doc = "0: Current range set to 0 - 1.6 uA."]
    RANGE0 = 0,
    #[doc = "1: Current range set to 1.6 - 4.7 uA."]
    RANGE1 = 1,
    #[doc = "2: Current range set to 0.5 - 16 uA."]
    RANGE2 = 2,
    #[doc = "3: Current range set to 2 - 64 uA."]
    RANGE3 = 3,
}
impl From<RANGESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RANGESEL_A {
    type Ux = u8;
}
impl RANGESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RANGESEL_A {
        match self.bits {
            0 => RANGESEL_A::RANGE0,
            1 => RANGESEL_A::RANGE1,
            2 => RANGESEL_A::RANGE2,
            3 => RANGESEL_A::RANGE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == RANGESEL_A::RANGE0
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == RANGESEL_A::RANGE1
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == RANGESEL_A::RANGE2
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == RANGESEL_A::RANGE3
    }
}
#[doc = "Field `RANGESEL` writer - Current Range Select"]
pub type RANGESEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RANGESEL_A>;
impl<'a, REG> RANGESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn range0(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL_A::RANGE0)
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL_A::RANGE1)
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL_A::RANGE2)
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL_A::RANGE3)
    }
}
#[doc = "Field `STEPSEL` reader - Current Step Size Select"]
pub type STEPSEL_R = crate::FieldReader;
#[doc = "Field `STEPSEL` writer - Current Step Size Select"]
pub type STEPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&self) -> RANGESEL_R {
        RANGESEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&self) -> STEPSEL_R {
        STEPSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    #[must_use]
    pub fn rangesel(&mut self) -> RANGESEL_W<CURPROG_SPEC> {
        RANGESEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn stepsel(&mut self) -> STEPSEL_W<CURPROG_SPEC> {
        STEPSEL_W::new(self, 8)
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
#[doc = "Current Programming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curprog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curprog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURPROG_SPEC;
impl crate::RegisterSpec for CURPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curprog::R`](R) reader structure"]
impl crate::Readable for CURPROG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`curprog::W`](W) writer structure"]
impl crate::Writable for CURPROG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURPROG to value 0"]
impl crate::Resettable for CURPROG_SPEC {
    const RESET_VALUE: u32 = 0;
}
