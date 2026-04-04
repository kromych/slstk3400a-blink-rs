#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `HFRCORDY` writer - HFRCO Ready Interrupt Flag Clear"]
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - HFXO Ready Interrupt Flag Clear"]
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - LFRCO Ready Interrupt Flag Clear"]
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - LFXO Ready Interrupt Flag Clear"]
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCO Ready Interrupt Flag Clear"]
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Flag Clear"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Flag Clear"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` writer - USHFRCO Ready Interrupt Flag Clear"]
pub type UshfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBCHFOSCSEL` writer - USBC HF-oscillator Selected Interrupt Flag Clear"]
pub type UsbchfoscselW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HfrcordyW<'_, IfcSpec> {
        HfrcordyW::new(self, 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HfxordyW<'_, IfcSpec> {
        HfxordyW::new(self, 1)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LfrcordyW<'_, IfcSpec> {
        LfrcordyW::new(self, 2)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LfxordyW<'_, IfcSpec> {
        LfxordyW::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<'_, IfcSpec> {
        AuxhfrcordyW::new(self, 4)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CalrdyW<'_, IfcSpec> {
        CalrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn calof(&mut self) -> CalofW<'_, IfcSpec> {
        CalofW::new(self, 6)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ushfrcordy(&mut self) -> UshfrcordyW<'_, IfcSpec> {
        UshfrcordyW::new(self, 8)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag Clear"]
    #[inline(always)]
    pub fn usbchfoscsel(&mut self) -> UsbchfoscselW<'_, IfcSpec> {
        UsbchfoscselW::new(self, 9)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
