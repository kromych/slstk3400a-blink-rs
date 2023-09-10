#[doc = "Register `INSENSE` reader"]
pub type R = crate::R<INSENSE_SPEC>;
#[doc = "Register `INSENSE` writer"]
pub type W = crate::W<INSENSE_SPEC>;
#[doc = "Field `INT` reader - Interrupt Sense Enable"]
pub type INT_R = crate::BitReader;
#[doc = "Field `INT` writer - Interrupt Sense Enable"]
pub type INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRS` reader - PRS Sense Enable"]
pub type PRS_R = crate::BitReader;
#[doc = "Field `PRS` writer - PRS Sense Enable"]
pub type PRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PRS Sense Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<INSENSE_SPEC, 0> {
        INT_W::new(self)
    }
    #[doc = "Bit 1 - PRS Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<INSENSE_SPEC, 1> {
        PRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input Sense Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`insense::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`insense::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INSENSE_SPEC;
impl crate::RegisterSpec for INSENSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`insense::R`](R) reader structure"]
impl crate::Readable for INSENSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`insense::W`](W) writer structure"]
impl crate::Writable for INSENSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSENSE to value 0x03"]
impl crate::Resettable for INSENSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
