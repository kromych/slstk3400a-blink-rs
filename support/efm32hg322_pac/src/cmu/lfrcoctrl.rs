#[doc = "Register `LFRCOCTRL` reader"]
pub type R = crate::R<LFRCOCTRL_SPEC>;
#[doc = "Register `LFRCOCTRL` writer"]
pub type W = crate::W<LFRCOCTRL_SPEC>;
#[doc = "Field `TUNING` reader - LFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader;
#[doc = "Field `TUNING` writer - LFRCO Tuning Value"]
pub type TUNING_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<LFRCOCTRL_SPEC> {
        TUNING_W::new(self, 0)
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
#[doc = "LFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrcoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrcoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFRCOCTRL_SPEC;
impl crate::RegisterSpec for LFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrcoctrl::R`](R) reader structure"]
impl crate::Readable for LFRCOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfrcoctrl::W`](W) writer structure"]
impl crate::Writable for LFRCOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFRCOCTRL to value 0x40"]
impl crate::Resettable for LFRCOCTRL_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
