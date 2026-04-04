#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<RouteSpec>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<RouteSpec>;
#[doc = "Field `CH0PEN` reader - CH0 Pin Enable"]
pub type Ch0penR = crate::BitReader;
#[doc = "Field `CH0PEN` writer - CH0 Pin Enable"]
pub type Ch0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PEN` reader - CH1 Pin Enable"]
pub type Ch1penR = crate::BitReader;
#[doc = "Field `CH1PEN` writer - CH1 Pin Enable"]
pub type Ch1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PEN` reader - CH2 Pin Enable"]
pub type Ch2penR = crate::BitReader;
#[doc = "Field `CH2PEN` writer - CH2 Pin Enable"]
pub type Ch2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PEN` reader - CH3 Pin Enable"]
pub type Ch3penR = crate::BitReader;
#[doc = "Field `CH3PEN` writer - CH3 Pin Enable"]
pub type Ch3penW<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl R {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&self) -> Ch0penR {
        Ch0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&self) -> Ch1penR {
        Ch1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&self) -> Ch2penR {
        Ch2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&self) -> Ch3penR {
        Ch3penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LocationR {
        LocationR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&mut self) -> Ch0penW<'_, RouteSpec> {
        Ch0penW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&mut self) -> Ch1penW<'_, RouteSpec> {
        Ch1penW::new(self, 1)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&mut self) -> Ch2penW<'_, RouteSpec> {
        Ch2penW::new(self, 2)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&mut self) -> Ch3penW<'_, RouteSpec> {
        Ch3penW::new(self, 3)
    }
    #[doc = "Bits 8:10 - I/O Location"]
    #[inline(always)]
    pub fn location(&mut self) -> LocationW<'_, RouteSpec> {
        LocationW::new(self, 8)
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
