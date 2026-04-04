#[doc = "Register `PE_DOUTTGL` writer"]
pub type W = crate::W<PeDouttglSpec>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DouttglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DouttglW<'_, PeDouttglSpec> {
        DouttglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeDouttglSpec;
impl crate::RegisterSpec for PeDouttglSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pe_douttgl::W`](W) writer structure"]
impl crate::Writable for PeDouttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_DOUTTGL to value 0"]
impl crate::Resettable for PeDouttglSpec {}
