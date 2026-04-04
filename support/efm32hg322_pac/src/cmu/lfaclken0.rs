#[doc = "Register `LFACLKEN0` reader"]
pub type R = crate::R<Lfaclken0Spec>;
#[doc = "Register `LFACLKEN0` writer"]
pub type W = crate::W<Lfaclken0Spec>;
#[doc = "Field `RTC` reader - Real-Time Counter Clock Enable"]
pub type RtcR = crate::BitReader;
#[doc = "Field `RTC` writer - Real-Time Counter Clock Enable"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Lfaclken0Spec> {
        RtcW::new(self, 0)
    }
}
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfaclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfaclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfaclken0Spec;
impl crate::RegisterSpec for Lfaclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclken0::R`](R) reader structure"]
impl crate::Readable for Lfaclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfaclken0::W`](W) writer structure"]
impl crate::Writable for Lfaclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFACLKEN0 to value 0"]
impl crate::Resettable for Lfaclken0Spec {}
