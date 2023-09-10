#[doc = "Register `DUTYCONFIG` reader"]
pub type R = crate::R<DUTYCONFIG_SPEC>;
#[doc = "Register `DUTYCONFIG` writer"]
pub type W = crate::W<DUTYCONFIG_SPEC>;
#[doc = "Field `DUTYCYCLEEN` reader - Duty Cycle Enable."]
pub type DUTYCYCLEEN_R = crate::BitReader;
#[doc = "Field `DUTYCYCLEEN` writer - Duty Cycle Enable."]
pub type DUTYCYCLEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM2DUTYCYCLEDIS` reader - EM2/EM3 Duty Cycle Disable."]
pub type EM2DUTYCYCLEDIS_R = crate::BitReader;
#[doc = "Field `EM2DUTYCYCLEDIS` writer - EM2/EM3 Duty Cycle Disable."]
pub type EM2DUTYCYCLEDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    pub fn dutycycleen(&self) -> DUTYCYCLEEN_R {
        DUTYCYCLEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    pub fn em2dutycycledis(&self) -> EM2DUTYCYCLEDIS_R {
        EM2DUTYCYCLEDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    #[must_use]
    pub fn dutycycleen(&mut self) -> DUTYCYCLEEN_W<DUTYCONFIG_SPEC, 0> {
        DUTYCYCLEEN_W::new(self)
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    #[must_use]
    pub fn em2dutycycledis(&mut self) -> EM2DUTYCYCLEDIS_W<DUTYCONFIG_SPEC, 1> {
        EM2DUTYCYCLEDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Duty Cycle Configauration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dutyconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dutyconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUTYCONFIG_SPEC;
impl crate::RegisterSpec for DUTYCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dutyconfig::R`](R) reader structure"]
impl crate::Readable for DUTYCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dutyconfig::W`](W) writer structure"]
impl crate::Writable for DUTYCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUTYCONFIG to value 0"]
impl crate::Resettable for DUTYCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
