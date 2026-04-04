#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `EN` writer - Enable DMA"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHPROT` writer - Channel Protection Control"]
pub type ChprotW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable DMA"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, ConfigSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 5 - Channel Protection Control"]
    #[inline(always)]
    pub fn chprot(&mut self) -> ChprotW<'_, ConfigSpec> {
        ChprotW::new(self, 5)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
