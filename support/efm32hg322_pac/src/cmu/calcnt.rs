#[doc = "Register `CALCNT` reader"]
pub type R = crate::R<CALCNT_SPEC>;
#[doc = "Register `CALCNT` writer"]
pub type W = crate::W<CALCNT_SPEC>;
#[doc = "Field `CALCNT` reader - Calibration Counter"]
pub type CALCNT_R = crate::FieldReader<u32>;
#[doc = "Field `CALCNT` writer - Calibration Counter"]
pub type CALCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    pub fn calcnt(&self) -> CALCNT_R {
        CALCNT_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    #[must_use]
    pub fn calcnt(&mut self) -> CALCNT_W<CALCNT_SPEC, 0> {
        CALCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Calibration Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALCNT_SPEC;
impl crate::RegisterSpec for CALCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calcnt::R`](R) reader structure"]
impl crate::Readable for CALCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calcnt::W`](W) writer structure"]
impl crate::Writable for CALCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALCNT to value 0"]
impl crate::Resettable for CALCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
