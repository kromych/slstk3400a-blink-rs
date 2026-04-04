#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `ERASE` reader - Erase Done Interrupt Enable"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - Erase Done Interrupt Enable"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - Write Done Interrupt Enable"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - Write Done Interrupt Enable"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` reader - Cache Hits Overflow Interrupt Enable"]
pub type ChofR = crate::BitReader;
#[doc = "Field `CHOF` writer - Cache Hits Overflow Interrupt Enable"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` reader - Cache Misses Overflow Interrupt Enable"]
pub type CmofR = crate::BitReader;
#[doc = "Field `CMOF` writer - Cache Misses Overflow Interrupt Enable"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Enable"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&self) -> ChofR {
        ChofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&self) -> CmofR {
        CmofR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase Done Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, IenSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Enable"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IenSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&mut self) -> ChofW<'_, IenSpec> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CmofW<'_, IenSpec> {
        CmofW::new(self, 3)
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
