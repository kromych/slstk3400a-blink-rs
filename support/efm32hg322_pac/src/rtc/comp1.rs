#[doc = "Register `COMP1` reader"]
pub type R = crate::R<COMP1_SPEC>;
#[doc = "Register `COMP1` writer"]
pub type W = crate::W<COMP1_SPEC>;
#[doc = "Field `COMP1` reader - Compare Value 1"]
pub type COMP1_R = crate::FieldReader<u32>;
#[doc = "Field `COMP1` writer - Compare Value 1"]
pub type COMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Compare Value 1"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<COMP1_SPEC> {
        COMP1_W::new(self, 0)
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
#[doc = "Compare Value Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP1_SPEC;
impl crate::RegisterSpec for COMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1::R`](R) reader structure"]
impl crate::Readable for COMP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp1::W`](W) writer structure"]
impl crate::Writable for COMP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1 to value 0"]
impl crate::Resettable for COMP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
