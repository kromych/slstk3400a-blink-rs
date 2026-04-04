#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Enable"]
pub type HfrcordyR = crate::BitReader;
#[doc = "Field `HFRCORDY` writer - HFRCO Ready Interrupt Enable"]
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Enable"]
pub type HfxordyR = crate::BitReader;
#[doc = "Field `HFXORDY` writer - HFXO Ready Interrupt Enable"]
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Enable"]
pub type LfrcordyR = crate::BitReader;
#[doc = "Field `LFRCORDY` writer - LFRCO Ready Interrupt Enable"]
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Enable"]
pub type LfxordyR = crate::BitReader;
#[doc = "Field `LFXORDY` writer - LFXO Ready Interrupt Enable"]
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Enable"]
pub type AuxhfrcordyR = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCO Ready Interrupt Enable"]
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Enable"]
pub type CalrdyR = crate::BitReader;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Enable"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Enable"]
pub type CalofR = crate::BitReader;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Enable"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready Interrupt Enable"]
pub type UshfrcordyR = crate::BitReader;
#[doc = "Field `USHFRCORDY` writer - USHFRCO Ready Interrupt Enable"]
pub type UshfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCHFOSCSEL` reader - USBC HF-oscillator Selected Interrupt Flag Clear"]
pub type UsbchfoscselR = crate::BitReader;
#[doc = "Field `USBCHFOSCSEL` writer - USBC HF-oscillator Selected Interrupt Flag Clear"]
pub type UsbchfoscselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> UshfrcordyR {
        UshfrcordyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Clear"]
    #[inline(always)]
    pub fn usbchfoscsel(&self) -> UsbchfoscselR {
        UsbchfoscselR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HfrcordyW<'_, IenSpec> {
        HfrcordyW::new(self, 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HfxordyW<'_, IenSpec> {
        HfxordyW::new(self, 1)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LfrcordyW<'_, IenSpec> {
        LfrcordyW::new(self, 2)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LfxordyW<'_, IenSpec> {
        LfxordyW::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<'_, IenSpec> {
        AuxhfrcordyW::new(self, 4)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CalrdyW<'_, IenSpec> {
        CalrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&mut self) -> CalofW<'_, IenSpec> {
        CalofW::new(self, 6)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&mut self) -> UshfrcordyW<'_, IenSpec> {
        UshfrcordyW::new(self, 8)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Clear"]
    #[inline(always)]
    pub fn usbchfoscsel(&mut self) -> UsbchfoscselW<'_, IenSpec> {
        UsbchfoscselW::new(self, 9)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
