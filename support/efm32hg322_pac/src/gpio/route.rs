#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<RouteSpec>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<RouteSpec>;
#[doc = "Field `SWCLKPEN` reader - Serial Wire Clock Pin Enable"]
pub type SwclkpenR = crate::BitReader;
#[doc = "Field `SWCLKPEN` writer - Serial Wire Clock Pin Enable"]
pub type SwclkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDIOPEN` reader - Serial Wire Data Pin Enable"]
pub type SwdiopenR = crate::BitReader;
#[doc = "Field `SWDIOPEN` writer - Serial Wire Data Pin Enable"]
pub type SwdiopenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    pub fn swclkpen(&self) -> SwclkpenR {
        SwclkpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    pub fn swdiopen(&self) -> SwdiopenR {
        SwdiopenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    pub fn swclkpen(&mut self) -> SwclkpenW<'_, RouteSpec> {
        SwclkpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    pub fn swdiopen(&mut self) -> SwdiopenW<'_, RouteSpec> {
        SwdiopenW::new(self, 1)
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
#[doc = "`reset()` method sets ROUTE to value 0x03"]
impl crate::Resettable for RouteSpec {
    const RESET_VALUE: u32 = 0x03;
}
