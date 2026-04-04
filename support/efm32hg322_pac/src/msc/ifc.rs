#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `ERASE` writer - Erase Done Interrupt Clear"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Write Done Interrupt Clear"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Cache Hits Overflow Interrupt Clear"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Cache Misses Overflow Interrupt Clear"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Erase Done Interrupt Clear"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, IfcSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Clear"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IfcSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn chof(&mut self) -> ChofW<'_, IfcSpec> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CmofW<'_, IfcSpec> {
        CmofW::new(self, 3)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
