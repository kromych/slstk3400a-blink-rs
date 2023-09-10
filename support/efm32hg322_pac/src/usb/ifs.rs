#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `VREGOSH` writer - Set VREGO Sense High Interrupt Flag"]
pub type VREGOSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREGOSL` writer - Set VREGO Sense Low Interrupt Flag"]
pub type VREGOSL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Set VREGO Sense High Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vregosh(&mut self) -> VREGOSH_W<IFS_SPEC, 0> {
        VREGOSH_W::new(self)
    }
    #[doc = "Bit 1 - Set VREGO Sense Low Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vregosl(&mut self) -> VREGOSL_W<IFS_SPEC, 1> {
        VREGOSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
