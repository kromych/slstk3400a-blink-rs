#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<ROUTE_SPEC>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<ROUTE_SPEC>;
#[doc = "Field `CC0PEN` reader - CC Channel 0 Pin Enable"]
pub type CC0PEN_R = crate::BitReader;
#[doc = "Field `CC0PEN` writer - CC Channel 0 Pin Enable"]
pub type CC0PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1PEN` reader - CC Channel 1 Pin Enable"]
pub type CC1PEN_R = crate::BitReader;
#[doc = "Field `CC1PEN` writer - CC Channel 1 Pin Enable"]
pub type CC1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2PEN` reader - CC Channel 2 Pin Enable"]
pub type CC2PEN_R = crate::BitReader;
#[doc = "Field `CC2PEN` writer - CC Channel 2 Pin Enable"]
pub type CC2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI0PEN` reader - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_R = crate::BitReader;
#[doc = "Field `CDTI0PEN` writer - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI1PEN` reader - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_R = crate::BitReader;
#[doc = "Field `CDTI1PEN` writer - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI2PEN` reader - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_R = crate::BitReader;
#[doc = "Field `CDTI2PEN` writer - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
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
            4 => Some(LOCATION_A::LOC4),
            5 => Some(LOCATION_A::LOC5),
            6 => Some(LOCATION_A::LOC6),
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
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == LOCATION_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == LOCATION_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == LOCATION_A::LOC6
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
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(LOCATION_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(LOCATION_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(LOCATION_A::LOC6)
    }
}
impl R {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&self) -> CC0PEN_R {
        CC0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&self) -> CC1PEN_R {
        CC1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&self) -> CC2PEN_R {
        CC2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&self) -> CDTI0PEN_R {
        CDTI0PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&self) -> CDTI1PEN_R {
        CDTI1PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&self) -> CDTI2PEN_R {
        CDTI2PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:18 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LOCATION_R {
        LOCATION_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc0pen(&mut self) -> CC0PEN_W<ROUTE_SPEC> {
        CC0PEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1pen(&mut self) -> CC1PEN_W<ROUTE_SPEC> {
        CC1PEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2pen(&mut self) -> CC2PEN_W<ROUTE_SPEC> {
        CC2PEN_W::new(self, 2)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdti0pen(&mut self) -> CDTI0PEN_W<ROUTE_SPEC> {
        CDTI0PEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdti1pen(&mut self) -> CDTI1PEN_W<ROUTE_SPEC> {
        CDTI1PEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdti2pen(&mut self) -> CDTI2PEN_W<ROUTE_SPEC> {
        CDTI2PEN_W::new(self, 10)
    }
    #[doc = "Bits 16:18 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn location(&mut self) -> LOCATION_W<ROUTE_SPEC> {
        LOCATION_W::new(self, 16)
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
