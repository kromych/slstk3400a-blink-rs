#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `VREGOSH` writer - Set VREGO Sense High Interrupt Flag"]
pub type VregoshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGOSL` writer - Set VREGO Sense Low Interrupt Flag"]
pub type VregoslW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set VREGO Sense High Interrupt Flag"]
    #[inline(always)]
    pub fn vregosh(&mut self) -> VregoshW<'_, IfsSpec> {
        VregoshW::new(self, 0)
    }
    #[doc = "Bit 1 - Set VREGO Sense Low Interrupt Flag"]
    #[inline(always)]
    pub fn vregosl(&mut self) -> VregoslW<'_, IfsSpec> {
        VregoslW::new(self, 1)
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
