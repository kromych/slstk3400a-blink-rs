#[doc = "Register `USHFRCOTUNE` reader"]
pub type R = crate::R<USHFRCOTUNE_SPEC>;
#[doc = "Register `USHFRCOTUNE` writer"]
pub type W = crate::W<USHFRCOTUNE_SPEC>;
#[doc = "Field `FINETUNING` reader - Oscillator fine frequency adjust"]
pub type FINETUNING_R = crate::FieldReader;
#[doc = "Field `FINETUNING` writer - Oscillator fine frequency adjust"]
pub type FINETUNING_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    pub fn finetuning(&self) -> FINETUNING_R {
        FINETUNING_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    #[must_use]
    pub fn finetuning(&mut self) -> FINETUNING_W<USHFRCOTUNE_SPEC> {
        FINETUNING_W::new(self, 0)
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
#[doc = "USHFRCO Frequency Tune\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcotune::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ushfrcotune::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USHFRCOTUNE_SPEC;
impl crate::RegisterSpec for USHFRCOTUNE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcotune::R`](R) reader structure"]
impl crate::Readable for USHFRCOTUNE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ushfrcotune::W`](W) writer structure"]
impl crate::Writable for USHFRCOTUNE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USHFRCOTUNE to value 0x20"]
impl crate::Resettable for USHFRCOTUNE_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
