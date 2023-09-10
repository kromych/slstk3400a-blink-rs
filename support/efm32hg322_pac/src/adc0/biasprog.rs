#[doc = "Register `BIASPROG` reader"]
pub type R = crate::R<BIASPROG_SPEC>;
#[doc = "Register `BIASPROG` writer"]
pub type W = crate::W<BIASPROG_SPEC>;
#[doc = "Field `BIASPROG` reader - Bias Programming Value"]
pub type BIASPROG_R = crate::FieldReader;
#[doc = "Field `BIASPROG` writer - Bias Programming Value"]
pub type BIASPROG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HALFBIAS_R = crate::BitReader;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HALFBIAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMPBIAS` reader - Comparator Bias Value"]
pub type COMPBIAS_R = crate::FieldReader;
#[doc = "Field `COMPBIAS` writer - Comparator Bias Value"]
pub type COMPBIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&self) -> COMPBIAS_R {
        COMPBIAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    #[must_use]
    pub fn biasprog(&mut self) -> BIASPROG_W<BIASPROG_SPEC, 0> {
        BIASPROG_W::new(self)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn halfbias(&mut self) -> HALFBIAS_W<BIASPROG_SPEC, 6> {
        HALFBIAS_W::new(self)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    #[must_use]
    pub fn compbias(&mut self) -> COMPBIAS_W<BIASPROG_SPEC, 8> {
        COMPBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bias Programming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasprog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasprog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASPROG_SPEC;
impl crate::RegisterSpec for BIASPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasprog::R`](R) reader structure"]
impl crate::Readable for BIASPROG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`biasprog::W`](W) writer structure"]
impl crate::Writable for BIASPROG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASPROG to value 0x0747"]
impl crate::Resettable for BIASPROG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0747;
}
