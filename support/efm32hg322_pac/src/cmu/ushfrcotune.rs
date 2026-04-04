#[doc = "Register `USHFRCOTUNE` reader"]
pub type R = crate::R<UshfrcotuneSpec>;
#[doc = "Register `USHFRCOTUNE` writer"]
pub type W = crate::W<UshfrcotuneSpec>;
#[doc = "Field `FINETUNING` reader - Oscillator fine frequency adjust"]
pub type FinetuningR = crate::FieldReader;
#[doc = "Field `FINETUNING` writer - Oscillator fine frequency adjust"]
pub type FinetuningW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    pub fn finetuning(&self) -> FinetuningR {
        FinetuningR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    pub fn finetuning(&mut self) -> FinetuningW<'_, UshfrcotuneSpec> {
        FinetuningW::new(self, 0)
    }
}
#[doc = "USHFRCO Frequency Tune\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcotune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ushfrcotune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UshfrcotuneSpec;
impl crate::RegisterSpec for UshfrcotuneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcotune::R`](R) reader structure"]
impl crate::Readable for UshfrcotuneSpec {}
#[doc = "`write(|w| ..)` method takes [`ushfrcotune::W`](W) writer structure"]
impl crate::Writable for UshfrcotuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USHFRCOTUNE to value 0x20"]
impl crate::Resettable for UshfrcotuneSpec {
    const RESET_VALUE: u32 = 0x20;
}
