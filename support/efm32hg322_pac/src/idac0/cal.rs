#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `TUNING` reader - Tune the current to given accuracy"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - Tune the current to given accuracy"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Tune the current to given accuracy"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tune the current to given accuracy"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, CalSpec> {
        TuningW::new(self, 0)
    }
}
#[doc = "Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0"]
impl crate::Resettable for CalSpec {}
