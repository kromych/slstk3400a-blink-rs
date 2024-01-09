#[doc = "Register `LFACLKEN0` reader"]
pub type R = crate::R<LFACLKEN0_SPEC>;
#[doc = "Register `LFACLKEN0` writer"]
pub type W = crate::W<LFACLKEN0_SPEC>;
#[doc = "Field `RTC` reader - Real-Time Counter Clock Enable"]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - Real-Time Counter Clock Enable"]
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<LFACLKEN0_SPEC> {
        RTC_W::new(self, 0)
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
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFACLKEN0_SPEC;
impl crate::RegisterSpec for LFACLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclken0::R`](R) reader structure"]
impl crate::Readable for LFACLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfaclken0::W`](W) writer structure"]
impl crate::Writable for LFACLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFACLKEN0 to value 0"]
impl crate::Resettable for LFACLKEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
