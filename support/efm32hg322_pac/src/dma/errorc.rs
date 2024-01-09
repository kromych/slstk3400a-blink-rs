#[doc = "Register `ERRORC` reader"]
pub type R = crate::R<ERRORC_SPEC>;
#[doc = "Register `ERRORC` writer"]
pub type W = crate::W<ERRORC_SPEC>;
#[doc = "Field `ERRORC` reader - Bus Error Clear"]
pub type ERRORC_R = crate::BitReader;
#[doc = "Field `ERRORC` writer - Bus Error Clear"]
pub type ERRORC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bus Error Clear"]
    #[inline(always)]
    pub fn errorc(&self) -> ERRORC_R {
        ERRORC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errorc(&mut self) -> ERRORC_W<ERRORC_SPEC> {
        ERRORC_W::new(self, 0)
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
#[doc = "Bus Error Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errorc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errorc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERRORC_SPEC;
impl crate::RegisterSpec for ERRORC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorc::R`](R) reader structure"]
impl crate::Readable for ERRORC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`errorc::W`](W) writer structure"]
impl crate::Writable for ERRORC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRORC to value 0"]
impl crate::Resettable for ERRORC_SPEC {
    const RESET_VALUE: u32 = 0;
}
