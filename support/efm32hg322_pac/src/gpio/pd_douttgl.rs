#[doc = "Register `PD_DOUTTGL` writer"]
pub type W = crate::W<PdDouttglSpec>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DouttglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DouttglW<'_, PdDouttglSpec> {
        DouttglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdDouttglSpec;
impl crate::RegisterSpec for PdDouttglSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pd_douttgl::W`](W) writer structure"]
impl crate::Writable for PdDouttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_DOUTTGL to value 0"]
impl crate::Resettable for PdDouttglSpec {}
