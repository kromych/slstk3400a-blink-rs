#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `EDGE` reader - Edge Trigger Interrupt Enable"]
pub type EDGE_R = crate::BitReader;
#[doc = "Field `EDGE` writer - Edge Trigger Interrupt Enable"]
pub type EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUP` reader - Warm-up Interrupt Enable"]
pub type WARMUP_R = crate::BitReader;
#[doc = "Field `WARMUP` writer - Warm-up Interrupt Enable"]
pub type WARMUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Edge Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Enable"]
    #[inline(always)]
    pub fn warmup(&self) -> WARMUP_R {
        WARMUP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<IEN_SPEC> {
        EDGE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn warmup(&mut self) -> WARMUP_W<IEN_SPEC> {
        WARMUP_W::new(self, 1)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
