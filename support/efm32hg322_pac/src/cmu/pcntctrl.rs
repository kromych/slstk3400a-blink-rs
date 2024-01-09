#[doc = "Register `PCNTCTRL` reader"]
pub type R = crate::R<PCNTCTRL_SPEC>;
#[doc = "Register `PCNTCTRL` writer"]
pub type W = crate::W<PCNTCTRL_SPEC>;
#[doc = "Field `PCNT0CLKEN` reader - PCNT0 Clock Enable"]
pub type PCNT0CLKEN_R = crate::BitReader;
#[doc = "Field `PCNT0CLKEN` writer - PCNT0 Clock Enable"]
pub type PCNT0CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0CLKSEL` reader - PCNT0 Clock Select"]
pub type PCNT0CLKSEL_R = crate::BitReader;
#[doc = "Field `PCNT0CLKSEL` writer - PCNT0 Clock Select"]
pub type PCNT0CLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> PCNT0CLKEN_R {
        PCNT0CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> PCNT0CLKSEL_R {
        PCNT0CLKSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clken(&mut self) -> PCNT0CLKEN_W<PCNTCTRL_SPEC> {
        PCNT0CLKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0clksel(&mut self) -> PCNT0CLKSEL_W<PCNTCTRL_SPEC> {
        PCNT0CLKSEL_W::new(self, 1)
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
#[doc = "PCNT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcntctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcntctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCNTCTRL_SPEC;
impl crate::RegisterSpec for PCNTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntctrl::R`](R) reader structure"]
impl crate::Readable for PCNTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcntctrl::W`](W) writer structure"]
impl crate::Writable for PCNTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCNTCTRL to value 0"]
impl crate::Resettable for PCNTCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
