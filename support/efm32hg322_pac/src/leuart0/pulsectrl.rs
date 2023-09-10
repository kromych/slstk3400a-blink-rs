#[doc = "Register `PULSECTRL` reader"]
pub type R = crate::R<PULSECTRL_SPEC>;
#[doc = "Register `PULSECTRL` writer"]
pub type W = crate::W<PULSECTRL_SPEC>;
#[doc = "Field `PULSEW` reader - Pulse Width"]
pub type PULSEW_R = crate::FieldReader;
#[doc = "Field `PULSEW` writer - Pulse Width"]
pub type PULSEW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PULSEEN` reader - Pulse Generator/Extender Enable"]
pub type PULSEEN_R = crate::BitReader;
#[doc = "Field `PULSEEN` writer - Pulse Generator/Extender Enable"]
pub type PULSEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PULSEFILT` reader - Pulse Filter"]
pub type PULSEFILT_R = crate::BitReader;
#[doc = "Field `PULSEFILT` writer - Pulse Filter"]
pub type PULSEFILT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&self) -> PULSEW_R {
        PULSEW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&self) -> PULSEEN_R {
        PULSEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&self) -> PULSEFILT_R {
        PULSEFILT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn pulsew(&mut self) -> PULSEW_W<PULSECTRL_SPEC, 0> {
        PULSEW_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pulseen(&mut self) -> PULSEEN_W<PULSECTRL_SPEC, 4> {
        PULSEEN_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    #[must_use]
    pub fn pulsefilt(&mut self) -> PULSEFILT_W<PULSECTRL_SPEC, 5> {
        PULSEFILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pulse Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulsectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulsectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULSECTRL_SPEC;
impl crate::RegisterSpec for PULSECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulsectrl::R`](R) reader structure"]
impl crate::Readable for PULSECTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pulsectrl::W`](W) writer structure"]
impl crate::Writable for PULSECTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULSECTRL to value 0"]
impl crate::Resettable for PULSECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
