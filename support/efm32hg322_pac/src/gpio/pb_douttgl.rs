#[doc = "Register `PB_DOUTTGL` writer"]
pub type W = crate::W<PbDouttglSpec>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DouttglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DouttglW<'_, PbDouttglSpec> {
        DouttglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbDouttglSpec;
impl crate::RegisterSpec for PbDouttglSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pb_douttgl::W`](W) writer structure"]
impl crate::Writable for PbDouttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_DOUTTGL to value 0"]
impl crate::Resettable for PbDouttglSpec {}
