#[doc = "Register `INPUTSEL` reader"]
pub type R = crate::R<INPUTSEL_SPEC>;
#[doc = "Register `INPUTSEL` writer"]
pub type W = crate::W<INPUTSEL_SPEC>;
#[doc = "Field `TRIGLEVEL` reader - Trigger Level"]
pub type TRIGLEVEL_R = crate::FieldReader;
#[doc = "Field `TRIGLEVEL` writer - Trigger Level"]
pub type TRIGLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LPREF` reader - Low Power Reference"]
pub type LPREF_R = crate::BitReader;
#[doc = "Field `LPREF` writer - Low Power Reference"]
pub type LPREF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    pub fn triglevel(&self) -> TRIGLEVEL_R {
        TRIGLEVEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    pub fn lpref(&self) -> LPREF_R {
        LPREF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn triglevel(&mut self) -> TRIGLEVEL_W<INPUTSEL_SPEC> {
        TRIGLEVEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Low Power Reference"]
    #[inline(always)]
    #[must_use]
    pub fn lpref(&mut self) -> LPREF_W<INPUTSEL_SPEC> {
        LPREF_W::new(self, 8)
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
#[doc = "Input Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTSEL_SPEC;
impl crate::RegisterSpec for INPUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputsel::R`](R) reader structure"]
impl crate::Readable for INPUTSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputsel::W`](W) writer structure"]
impl crate::Writable for INPUTSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPUTSEL to value 0"]
impl crate::Resettable for INPUTSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
