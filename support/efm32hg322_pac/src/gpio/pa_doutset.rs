#[doc = "Register `PA_DOUTSET` writer"]
pub type W = crate::W<PaDoutsetSpec>;
#[doc = "Field `DOUTSET` writer - Data Out Set"]
pub type DoutsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Set"]
    #[inline(always)]
    pub fn doutset(&mut self) -> DoutsetW<'_, PaDoutsetSpec> {
        DoutsetW::new(self, 0)
    }
}
#[doc = "Port Data Out Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_doutset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaDoutsetSpec;
impl crate::RegisterSpec for PaDoutsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pa_doutset::W`](W) writer structure"]
impl crate::Writable for PaDoutsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_DOUTSET to value 0"]
impl crate::Resettable for PaDoutsetSpec {}
