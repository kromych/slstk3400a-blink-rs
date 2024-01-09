#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<ROUTE_SPEC>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<ROUTE_SPEC>;
#[doc = "Field `PHYPEN` reader - USB PHY Pin Enable"]
pub type PHYPEN_R = crate::BitReader;
#[doc = "Field `PHYPEN` writer - USB PHY Pin Enable"]
pub type PHYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMPUPEN` reader - DMPU Pin Enable"]
pub type DMPUPEN_R = crate::BitReader;
#[doc = "Field `DMPUPEN` writer - DMPU Pin Enable"]
pub type DMPUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    pub fn phypen(&self) -> PHYPEN_R {
        PHYPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    pub fn dmpupen(&self) -> DMPUPEN_R {
        DMPUPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phypen(&mut self) -> PHYPEN_W<ROUTE_SPEC> {
        PHYPEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - DMPU Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmpupen(&mut self) -> DMPUPEN_W<ROUTE_SPEC> {
        DMPUPEN_W::new(self, 2)
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
