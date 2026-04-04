#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<RouteSpec>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<RouteSpec>;
#[doc = "Field `CC0PEN` reader - CC Channel 0 Pin Enable"]
pub type Cc0penR = crate::BitReader;
#[doc = "Field `CC0PEN` writer - CC Channel 0 Pin Enable"]
pub type Cc0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1PEN` reader - CC Channel 1 Pin Enable"]
pub type Cc1penR = crate::BitReader;
#[doc = "Field `CC1PEN` writer - CC Channel 1 Pin Enable"]
pub type Cc1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2PEN` reader - CC Channel 2 Pin Enable"]
pub type Cc2penR = crate::BitReader;
#[doc = "Field `CC2PEN` writer - CC Channel 2 Pin Enable"]
pub type Cc2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI0PEN` reader - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti0penR = crate::BitReader;
#[doc = "Field `CDTI0PEN` writer - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI1PEN` reader - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti1penR = crate::BitReader;
#[doc = "Field `CDTI1PEN` writer - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI2PEN` reader - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti2penR = crate::BitReader;
#[doc = "Field `CDTI2PEN` writer - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Location {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
}
impl From<Location> for u8 {
    #[inline(always)]
    fn from(variant: Location) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Location {
    type Ux = u8;
}
impl crate::IsEnum for Location {}
#[doc = "Field `LOCATION` reader - I/O Location"]
pub type LocationR = crate::FieldReader<Location>;
impl LocationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Location> {
        match self.bits {
            0 => Some(Location::Loc0),
            1 => Some(Location::Loc1),
            2 => Some(Location::Loc2),
            3 => Some(Location::Loc3),
            4 => Some(Location::Loc4),
            5 => Some(Location::Loc5),
            6 => Some(Location::Loc6),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Location::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Location::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Location::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Location::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Location::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == Location::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == Location::Loc6
    }
}
#[doc = "Field `LOCATION` writer - I/O Location"]
pub type LocationW<'a, REG> = crate::FieldWriter<'a, REG, 3, Location>;
impl<'a, REG> LocationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Location::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Location::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Location::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Location::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Location::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(Location::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(Location::Loc6)
    }
}
impl R {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&self) -> Cc0penR {
        Cc0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&self) -> Cc1penR {
        Cc1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&self) -> Cc2penR {
        Cc2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&self) -> Cdti0penR {
        Cdti0penR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&self) -> Cdti1penR {
        Cdti1penR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&self) -> Cdti2penR {
        Cdti2penR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:18 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LocationR {
        LocationR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&mut self) -> Cc0penW<'_, RouteSpec> {
        Cc0penW::new(self, 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&mut self) -> Cc1penW<'_, RouteSpec> {
        Cc1penW::new(self, 1)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&mut self) -> Cc2penW<'_, RouteSpec> {
        Cc2penW::new(self, 2)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&mut self) -> Cdti0penW<'_, RouteSpec> {
        Cdti0penW::new(self, 8)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&mut self) -> Cdti1penW<'_, RouteSpec> {
        Cdti1penW::new(self, 9)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&mut self) -> Cdti2penW<'_, RouteSpec> {
        Cdti2penW::new(self, 10)
    }
    #[doc = "Bits 16:18 - I/O Location"]
    #[inline(always)]
    pub fn location(&mut self) -> LocationW<'_, RouteSpec> {
        LocationW::new(self, 16)
    }
}
#[doc = "I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RouteSpec;
impl crate::RegisterSpec for RouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`route::R`](R) reader structure"]
impl crate::Readable for RouteSpec {}
#[doc = "`write(|w| ..)` method takes [`route::W`](W) writer structure"]
impl crate::Writable for RouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTE to value 0"]
impl crate::Resettable for RouteSpec {}
