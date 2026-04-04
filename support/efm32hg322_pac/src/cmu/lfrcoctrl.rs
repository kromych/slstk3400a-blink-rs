#[doc = "Register `LFRCOCTRL` reader"]
pub type R = crate::R<LfrcoctrlSpec>;
#[doc = "Register `LFRCOCTRL` writer"]
pub type W = crate::W<LfrcoctrlSpec>;
#[doc = "Field `TUNING` reader - LFRCO Tuning Value"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - LFRCO Tuning Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, LfrcoctrlSpec> {
        TuningW::new(self, 0)
    }
}
#[doc = "LFRCO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfrcoctrlSpec;
impl crate::RegisterSpec for LfrcoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrcoctrl::R`](R) reader structure"]
impl crate::Readable for LfrcoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfrcoctrl::W`](W) writer structure"]
impl crate::Writable for LfrcoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFRCOCTRL to value 0x40"]
impl crate::Resettable for LfrcoctrlSpec {
    const RESET_VALUE: u32 = 0x40;
}
