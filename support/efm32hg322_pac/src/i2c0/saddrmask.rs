#[doc = "Register `SADDRMASK` reader"]
pub type R = crate::R<SADDRMASK_SPEC>;
#[doc = "Register `SADDRMASK` writer"]
pub type W = crate::W<SADDRMASK_SPEC>;
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub type MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<SADDRMASK_SPEC, 1> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddrmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddrmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDRMASK_SPEC;
impl crate::RegisterSpec for SADDRMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddrmask::R`](R) reader structure"]
impl crate::Readable for SADDRMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddrmask::W`](W) writer structure"]
impl crate::Writable for SADDRMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::Resettable for SADDRMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
