#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<ROUTE_SPEC>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<ROUTE_SPEC>;
#[doc = "Field `ACMPPEN` reader - ACMP Output Pin Enable"]
pub type ACMPPEN_R = crate::BitReader;
#[doc = "Field `ACMPPEN` writer - ACMP Output Pin Enable"]
pub type ACMPPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION` reader - I/O Location"]
pub type LOCATION_R = crate::FieldReader<LOCATION_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCATION_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<LOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCATION_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCATION_A {
    type Ux = u8;
}
impl LOCATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LOCATION_A> {
        match self.bits {
            0 => Some(LOCATION_A::LOC0),
            1 => Some(LOCATION_A::LOC1),
            2 => Some(LOCATION_A::LOC2),
            3 => Some(LOCATION_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == LOCATION_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == LOCATION_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == LOCATION_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == LOCATION_A::LOC3
    }
}
#[doc = "Field `LOCATION` writer - I/O Location"]
pub type LOCATION_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LOCATION_A>;
impl<'a, REG> LOCATION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCATION_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCATION_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCATION_A::LOC3)
    }
}
impl R {
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn acmppen(&self) -> ACMPPEN_R {
        ACMPPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LOCATION_R {
        LOCATION_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmppen(&mut self) -> ACMPPEN_W<ROUTE_SPEC> {
        ACMPPEN_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn location(&mut self) -> LOCATION_W<ROUTE_SPEC> {
        LOCATION_W::new(self, 8)
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
#[doc = "I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTE_SPEC;
impl crate::RegisterSpec for ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`route::R`](R) reader structure"]
impl crate::Readable for ROUTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`route::W`](W) writer structure"]
impl crate::Writable for ROUTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTE to value 0"]
impl crate::Resettable for ROUTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
