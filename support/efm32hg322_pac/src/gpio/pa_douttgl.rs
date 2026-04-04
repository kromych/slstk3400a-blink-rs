#[doc = "Register `PA_DOUTTGL` writer"]
pub type W = crate::W<PaDouttglSpec>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DouttglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DouttglW<'_, PaDouttglSpec> {
        DouttglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaDouttglSpec;
impl crate::RegisterSpec for PaDouttglSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pa_douttgl::W`](W) writer structure"]
impl crate::Writable for PaDouttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_DOUTTGL to value 0"]
impl crate::Resettable for PaDouttglSpec {}
