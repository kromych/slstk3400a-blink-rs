#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Flag"]
pub type HfrcordyR = crate::BitReader;
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Flag"]
pub type HfxordyR = crate::BitReader;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Flag"]
pub type LfrcordyR = crate::BitReader;
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Flag"]
pub type LfxordyR = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Flag"]
pub type AuxhfrcordyR = crate::BitReader;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Flag"]
pub type CalrdyR = crate::BitReader;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Flag"]
pub type CalofR = crate::BitReader;
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready Interrupt Flag"]
pub type UshfrcordyR = crate::BitReader;
#[doc = "Field `USBCHFOSCSEL` reader - USBC HF-oscillator Selected Interrupt Flag"]
pub type UsbchfoscselR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> UshfrcordyR {
        UshfrcordyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag"]
    #[inline(always)]
    pub fn usbchfoscsel(&self) -> UsbchfoscselR {
        UsbchfoscselR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0x01"]
impl crate::Resettable for IfSpec {
    const RESET_VALUE: u32 = 0x01;
}
