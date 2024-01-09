#[doc = "Register `PB_DOUTCLR` writer"]
pub type W = crate::W<PB_DOUTCLR_SPEC>;
#[doc = "Field `DOUTCLR` writer - Data Out Clear"]
pub type DOUTCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Clear"]
    #[inline(always)]
    #[must_use]
    pub fn doutclr(&mut self) -> DOUTCLR_W<PB_DOUTCLR_SPEC> {
        DOUTCLR_W::new(self, 0)
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
#[doc = "Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_doutclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_DOUTCLR_SPEC;
impl crate::RegisterSpec for PB_DOUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pb_doutclr::W`](W) writer structure"]
impl crate::Writable for PB_DOUTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB_DOUTCLR to value 0"]
impl crate::Resettable for PB_DOUTCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
