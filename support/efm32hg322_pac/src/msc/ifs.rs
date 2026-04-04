#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `ERASE` writer - Erase Done Interrupt Set"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Write Done Interrupt Set"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Cache Hits Overflow Interrupt Set"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Cache Misses Overflow Interrupt Set"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Erase Done Interrupt Set"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, IfsSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Set"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IfsSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Set"]
    #[inline(always)]
    pub fn chof(&mut self) -> ChofW<'_, IfsSpec> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Set"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CmofW<'_, IfsSpec> {
        CmofW::new(self, 3)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
