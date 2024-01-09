#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `HFRCORDY` writer - HFRCO Ready Interrupt Flag Set"]
pub type HFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - HFXO Ready Interrupt Flag Set"]
pub type HFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - LFRCO Ready Interrupt Flag Set"]
pub type LFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - LFXO Ready Interrupt Flag Set"]
pub type LFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCO Ready Interrupt Flag Set"]
pub type AUXHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Flag Set"]
pub type CALRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Flag Set"]
pub type CALOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` writer - USHFRCO Ready Interrupt Flag Set"]
pub type USHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCHFOSCSEL` writer - USBC HF-oscillator Selected Interrupt Flag Set"]
pub type USBCHFOSCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<IFS_SPEC> {
        HFRCORDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<IFS_SPEC> {
        HFXORDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<IFS_SPEC> {
        LFRCORDY_W::new(self, 2)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<IFS_SPEC> {
        LFXORDY_W::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<IFS_SPEC> {
        AUXHFRCORDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<IFS_SPEC> {
        CALRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<IFS_SPEC> {
        CALOF_W::new(self, 6)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcordy(&mut self) -> USHFRCORDY_W<IFS_SPEC> {
        USHFRCORDY_W::new(self, 8)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn usbchfoscsel(&mut self) -> USBCHFOSCSEL_W<IFS_SPEC> {
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
