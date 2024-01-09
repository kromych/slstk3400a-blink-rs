#[doc = "Register `AUXCTRL` reader"]
pub type R = crate::R<AUXCTRL_SPEC>;
#[doc = "Register `AUXCTRL` writer"]
pub type W = crate::W<AUXCTRL_SPEC>;
#[doc = "Field `HRCCLR` reader - Hard Reset Cause Clear"]
pub type HRCCLR_R = crate::BitReader;
#[doc = "Field `HRCCLR` writer - Hard Reset Cause Clear"]
pub type HRCCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    pub fn hrcclr(&self) -> HRCCLR_R {
        HRCCLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hard Reset Cause Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hrcclr(&mut self) -> HRCCLR_W<AUXCTRL_SPEC> {
        HRCCLR_W::new(self, 0)
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
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUXCTRL_SPEC;
impl crate::RegisterSpec for AUXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxctrl::R`](R) reader structure"]
impl crate::Readable for AUXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`auxctrl::W`](W) writer structure"]
impl crate::Writable for AUXCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXCTRL to value 0"]
impl crate::Resettable for AUXCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
