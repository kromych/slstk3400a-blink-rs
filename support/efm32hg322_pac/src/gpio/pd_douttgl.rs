#[doc = "Register `PD_DOUTTGL` writer"]
pub type W = crate::W<PD_DOUTTGL_SPEC>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DOUTTGL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl(&mut self) -> DOUTTGL_W<PD_DOUTTGL_SPEC> {
        DOUTTGL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_DOUTTGL_SPEC;
impl crate::RegisterSpec for PD_DOUTTGL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pd_douttgl::W`](W) writer structure"]
impl crate::Writable for PD_DOUTTGL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD_DOUTTGL to value 0"]
impl crate::Resettable for PD_DOUTTGL_SPEC {
    const RESET_VALUE: u32 = 0;
}
