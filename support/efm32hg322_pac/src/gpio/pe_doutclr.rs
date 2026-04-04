#[doc = "Register `PE_DOUTCLR` writer"]
pub type W = crate::W<PeDoutclrSpec>;
#[doc = "Field `DOUTCLR` writer - Data Out Clear"]
pub type DoutclrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Clear"]
    #[inline(always)]
    pub fn doutclr(&mut self) -> DoutclrW<'_, PeDoutclrSpec> {
        DoutclrW::new(self, 0)
    }
}
#[doc = "Port Data Out Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_doutclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeDoutclrSpec;
impl crate::RegisterSpec for PeDoutclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pe_doutclr::W`](W) writer structure"]
impl crate::Writable for PeDoutclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_DOUTCLR to value 0"]
impl crate::Resettable for PeDoutclrSpec {}
