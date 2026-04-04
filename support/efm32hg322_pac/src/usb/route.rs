#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<RouteSpec>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<RouteSpec>;
#[doc = "Field `PHYPEN` reader - USB PHY Pin Enable"]
pub type PhypenR = crate::BitReader;
#[doc = "Field `PHYPEN` writer - USB PHY Pin Enable"]
pub type PhypenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMPUPEN` reader - DMPU Pin Enable"]
pub type DmpupenR = crate::BitReader;
#[doc = "Field `DMPUPEN` writer - DMPU Pin Enable"]
pub type DmpupenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    pub fn phypen(&self) -> PhypenR {
        PhypenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    pub fn dmpupen(&self) -> DmpupenR {
        DmpupenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    pub fn phypen(&mut self) -> PhypenW<'_, RouteSpec> {
        PhypenW::new(self, 0)
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    pub fn dmpupen(&mut self) -> DmpupenW<'_, RouteSpec> {
        DmpupenW::new(self, 2)
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
