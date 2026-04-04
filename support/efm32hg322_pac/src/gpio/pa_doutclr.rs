#[doc = "Register `PA_DOUTCLR` writer"]
pub type W = crate::W<PaDoutclrSpec>;
#[doc = "Field `DOUTCLR` writer - Data Out Clear"]
pub type DoutclrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Clear"]
    #[inline(always)]
    pub fn doutclr(&mut self) -> DoutclrW<'_, PaDoutclrSpec> {
        DoutclrW::new(self, 0)
    }
}
#[doc = "Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_doutclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaDoutclrSpec;
impl crate::RegisterSpec for PaDoutclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pa_doutclr::W`](W) writer structure"]
impl crate::Writable for PaDoutclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_DOUTCLR to value 0"]
impl crate::Resettable for PaDoutclrSpec {}
