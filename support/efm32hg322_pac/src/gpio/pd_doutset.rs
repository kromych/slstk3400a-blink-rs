#[doc = "Register `PD_DOUTSET` writer"]
pub type W = crate::W<PdDoutsetSpec>;
#[doc = "Field `DOUTSET` writer - Data Out Set"]
pub type DoutsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Set"]
    #[inline(always)]
    pub fn doutset(&mut self) -> DoutsetW<'_, PdDoutsetSpec> {
        DoutsetW::new(self, 0)
    }
}
#[doc = "Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_doutset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdDoutsetSpec;
impl crate::RegisterSpec for PdDoutsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pd_doutset::W`](W) writer structure"]
impl crate::Writable for PdDoutsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_DOUTSET to value 0"]
impl crate::Resettable for PdDoutsetSpec {}
