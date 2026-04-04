#[doc = "Register `PF_DOUTTGL` writer"]
pub type W = crate::W<PfDouttglSpec>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DouttglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DouttglW<'_, PfDouttglSpec> {
        DouttglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfDouttglSpec;
impl crate::RegisterSpec for PfDouttglSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pf_douttgl::W`](W) writer structure"]
impl crate::Writable for PfDouttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_DOUTTGL to value 0"]
impl crate::Resettable for PfDouttglSpec {}
