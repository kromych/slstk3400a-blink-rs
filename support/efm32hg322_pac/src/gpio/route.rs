#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<ROUTE_SPEC>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<ROUTE_SPEC>;
#[doc = "Field `SWCLKPEN` reader - Serial Wire Clock Pin Enable"]
pub type SWCLKPEN_R = crate::BitReader;
#[doc = "Field `SWCLKPEN` writer - Serial Wire Clock Pin Enable"]
pub type SWCLKPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDIOPEN` reader - Serial Wire Data Pin Enable"]
pub type SWDIOPEN_R = crate::BitReader;
#[doc = "Field `SWDIOPEN` writer - Serial Wire Data Pin Enable"]
pub type SWDIOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    pub fn swclkpen(&self) -> SWCLKPEN_R {
        SWCLKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    pub fn swdiopen(&self) -> SWDIOPEN_R {
        SWDIOPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swclkpen(&mut self) -> SWCLKPEN_W<ROUTE_SPEC> {
        SWCLKPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swdiopen(&mut self) -> SWDIOPEN_W<ROUTE_SPEC> {
        SWDIOPEN_W::new(self, 1)
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
#[doc = "`reset()` method sets ROUTE to value 0x03"]
impl crate::Resettable for ROUTE_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
