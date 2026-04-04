#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `EDGE` reader - Edge Trigger Interrupt Enable"]
pub type EdgeR = crate::BitReader;
#[doc = "Field `EDGE` writer - Edge Trigger Interrupt Enable"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUP` reader - Warm-up Interrupt Enable"]
pub type WarmupR = crate::BitReader;
#[doc = "Field `WARMUP` writer - Warm-up Interrupt Enable"]
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Edge Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Enable"]
    #[inline(always)]
    pub fn warmup(&self) -> WarmupR {
        WarmupR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<'_, IenSpec> {
        EdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Enable"]
    #[inline(always)]
    pub fn warmup(&mut self) -> WarmupW<'_, IenSpec> {
        WarmupW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
