#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `EDGE` writer - Edge Triggered Interrupt Flag Clear"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUP` writer - Warm-up Interrupt Flag Clear"]
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Edge Triggered Interrupt Flag Clear"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<'_, IfcSpec> {
        EdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Flag Clear"]
    #[inline(always)]
    pub fn warmup(&mut self) -> WarmupW<'_, IfcSpec> {
        WarmupW::new(self, 1)
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
