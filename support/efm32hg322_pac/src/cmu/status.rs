#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `HFRCOENS` reader - HFRCO Enable Status"]
pub type HfrcoensR = crate::BitReader;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready"]
pub type HfrcordyR = crate::BitReader;
#[doc = "Field `HFXOENS` reader - HFXO Enable Status"]
pub type HfxoensR = crate::BitReader;
#[doc = "Field `HFXORDY` reader - HFXO Ready"]
pub type HfxordyR = crate::BitReader;
#[doc = "Field `AUXHFRCOENS` reader - AUXHFRCO Enable Status"]
pub type AuxhfrcoensR = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready"]
pub type AuxhfrcordyR = crate::BitReader;
#[doc = "Field `LFRCOENS` reader - LFRCO Enable Status"]
pub type LfrcoensR = crate::BitReader;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready"]
pub type LfrcordyR = crate::BitReader;
#[doc = "Field `LFXOENS` reader - LFXO Enable Status"]
pub type LfxoensR = crate::BitReader;
#[doc = "Field `LFXORDY` reader - LFXO Ready"]
pub type LfxordyR = crate::BitReader;
#[doc = "Field `HFRCOSEL` reader - HFRCO Selected"]
pub type HfrcoselR = crate::BitReader;
#[doc = "Field `HFXOSEL` reader - HFXO Selected"]
pub type HfxoselR = crate::BitReader;
#[doc = "Field `LFRCOSEL` reader - LFRCO Selected"]
pub type LfrcoselR = crate::BitReader;
#[doc = "Field `LFXOSEL` reader - LFXO Selected"]
pub type LfxoselR = crate::BitReader;
#[doc = "Field `CALBSY` reader - Calibration Busy"]
pub type CalbsyR = crate::BitReader;
#[doc = "Field `USBCLFXOSEL` reader - USBC LFXO Selected"]
pub type UsbclfxoselR = crate::BitReader;
#[doc = "Field `USBCLFRCOSEL` reader - USBC LFRCO Selected"]
pub type UsbclfrcoselR = crate::BitReader;
#[doc = "Field `USBCUSHFRCOSEL` reader - USBC USHFRCO Selected"]
pub type UsbcushfrcoselR = crate::BitReader;
#[doc = "Field `USBCHFCLKSYNC` reader - USBC is synchronous to HFCLK"]
pub type UsbchfclksyncR = crate::BitReader;
#[doc = "Field `USHFRCOENS` reader - USHFRCO Enable Status"]
pub type UshfrcoensR = crate::BitReader;
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready"]
pub type UshfrcordyR = crate::BitReader;
#[doc = "Field `USHFRCOSUSPEND` reader - USHFRCO is suspended"]
pub type UshfrcosuspendR = crate::BitReader;
#[doc = "Field `USHFRCODIV2SEL` reader - USHFRCODIV2 Selected"]
pub type Ushfrcodiv2selR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HfrcoensR {
        HfrcoensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HfxoensR {
        HfxoensR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AuxhfrcoensR {
        AuxhfrcoensR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LfrcoensR {
        LfrcoensR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LfxoensR {
        LfxoensR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HFRCO Selected"]
    #[inline(always)]
    pub fn hfrcosel(&self) -> HfrcoselR {
        HfrcoselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXO Selected"]
    #[inline(always)]
    pub fn hfxosel(&self) -> HfxoselR {
        HfxoselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LFRCO Selected"]
    #[inline(always)]
    pub fn lfrcosel(&self) -> LfrcoselR {
        LfrcoselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LFXO Selected"]
    #[inline(always)]
    pub fn lfxosel(&self) -> LfxoselR {
        LfxoselR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Calibration Busy"]
    #[inline(always)]
    pub fn calbsy(&self) -> CalbsyR {
        CalbsyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - USBC LFXO Selected"]
    #[inline(always)]
    pub fn usbclfxosel(&self) -> UsbclfxoselR {
        UsbclfxoselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USBC LFRCO Selected"]
    #[inline(always)]
    pub fn usbclfrcosel(&self) -> UsbclfrcoselR {
        UsbclfrcoselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USBC USHFRCO Selected"]
    #[inline(always)]
    pub fn usbcushfrcosel(&self) -> UsbcushfrcoselR {
        UsbcushfrcoselR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - USBC is synchronous to HFCLK"]
    #[inline(always)]
    pub fn usbchfclksync(&self) -> UsbchfclksyncR {
        UsbchfclksyncR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USHFRCO Enable Status"]
    #[inline(always)]
    pub fn ushfrcoens(&self) -> UshfrcoensR {
        UshfrcoensR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - USHFRCO Ready"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> UshfrcordyR {
        UshfrcordyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USHFRCO is suspended"]
    #[inline(always)]
    pub fn ushfrcosuspend(&self) -> UshfrcosuspendR {
        UshfrcosuspendR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - USHFRCODIV2 Selected"]
    #[inline(always)]
    pub fn ushfrcodiv2sel(&self) -> Ushfrcodiv2selR {
        Ushfrcodiv2selR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x0403"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0403;
}
