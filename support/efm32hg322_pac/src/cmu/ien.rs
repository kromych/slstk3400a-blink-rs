#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Enable"]
pub type HFRCORDY_R = crate::BitReader;
#[doc = "Field `HFRCORDY` writer - HFRCO Ready Interrupt Enable"]
pub type HFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Enable"]
pub type HFXORDY_R = crate::BitReader;
#[doc = "Field `HFXORDY` writer - HFXO Ready Interrupt Enable"]
pub type HFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Enable"]
pub type LFRCORDY_R = crate::BitReader;
#[doc = "Field `LFRCORDY` writer - LFRCO Ready Interrupt Enable"]
pub type LFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Enable"]
pub type LFXORDY_R = crate::BitReader;
#[doc = "Field `LFXORDY` writer - LFXO Ready Interrupt Enable"]
pub type LFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Enable"]
pub type AUXHFRCORDY_R = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCO Ready Interrupt Enable"]
pub type AUXHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Enable"]
pub type CALRDY_R = crate::BitReader;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Enable"]
pub type CALRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Enable"]
pub type CALOF_R = crate::BitReader;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Enable"]
pub type CALOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready Interrupt Enable"]
pub type USHFRCORDY_R = crate::BitReader;
#[doc = "Field `USHFRCORDY` writer - USHFRCO Ready Interrupt Enable"]
pub type USHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCHFOSCSEL` reader - USBC HF-oscillator Selected Interrupt Flag Clear"]
pub type USBCHFOSCSEL_R = crate::BitReader;
#[doc = "Field `USBCHFOSCSEL` writer - USBC HF-oscillator Selected Interrupt Flag Clear"]
pub type USBCHFOSCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Clear"]
    #[inline(always)]
    pub fn usbchfoscsel(&self) -> USBCHFOSCSEL_R {
        USBCHFOSCSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<IEN_SPEC> {
        HFRCORDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<IEN_SPEC> {
        HFXORDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<IEN_SPEC> {
        LFRCORDY_W::new(self, 2)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<IEN_SPEC> {
        LFXORDY_W::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<IEN_SPEC> {
        AUXHFRCORDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<IEN_SPEC> {
        CALRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<IEN_SPEC> {
        CALOF_W::new(self, 6)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcordy(&mut self) -> USHFRCORDY_W<IEN_SPEC> {
        USHFRCORDY_W::new(self, 8)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbchfoscsel(&mut self) -> USBCHFOSCSEL_W<IEN_SPEC> {
        USBCHFOSCSEL_W::new(self, 9)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
