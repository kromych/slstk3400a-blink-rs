#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `VREGOSH` writer - Clear VREGO Sense High Interrupt Flag"]
pub type VregoshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGOSL` writer - Clear VREGO Sense Low Interrupt Flag"]
pub type VregoslW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear VREGO Sense High Interrupt Flag"]
    #[inline(always)]
    pub fn vregosh(&mut self) -> VregoshW<'_, IfcSpec> {
        VregoshW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear VREGO Sense Low Interrupt Flag"]
    #[inline(always)]
    pub fn vregosl(&mut self) -> VregoslW<'_, IfcSpec> {
        VregoslW::new(self, 1)
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
