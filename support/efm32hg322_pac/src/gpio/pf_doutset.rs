#[doc = "Register `PF_DOUTSET` writer"]
pub type W = crate::W<PF_DOUTSET_SPEC>;
#[doc = "Field `DOUTSET` writer - Data Out Set"]
pub type DOUTSET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Set"]
    #[inline(always)]
    #[must_use]
    pub fn doutset(&mut self) -> DOUTSET_W<PF_DOUTSET_SPEC> {
        DOUTSET_W::new(self, 0)
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
#[doc = "Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_doutset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_DOUTSET_SPEC;
impl crate::RegisterSpec for PF_DOUTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pf_doutset::W`](W) writer structure"]
impl crate::Writable for PF_DOUTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PF_DOUTSET to value 0"]
impl crate::Resettable for PF_DOUTSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
