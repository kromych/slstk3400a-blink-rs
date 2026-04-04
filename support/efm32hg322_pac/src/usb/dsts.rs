#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DstsSpec>;
#[doc = "Field `SUSPSTS` reader - Suspend Status"]
pub type SuspstsR = crate::BitReader;
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enumspd {
    #[doc = "2: Low speed (PHY clock is running at 6 MHz)."]
    Ls = 2,
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)."]
    Fs = 3,
}
impl From<Enumspd> for u8 {
    #[inline(always)]
    fn from(variant: Enumspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enumspd {
    type Ux = u8;
}
impl crate::IsEnum for Enumspd {}
#[doc = "Field `ENUMSPD` reader - Enumerated Speed"]
pub type EnumspdR = crate::FieldReader<Enumspd>;
impl EnumspdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enumspd> {
        match self.bits {
            2 => Some(Enumspd::Ls),
            3 => Some(Enumspd::Fs),
            _ => None,
        }
    }
    #[doc = "Low speed (PHY clock is running at 6 MHz)."]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == Enumspd::Ls
    }
    #[doc = "Full speed (PHY clock is running at 48 MHz)."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == Enumspd::Fs
    }
}
#[doc = "Field `ERRTICERR` reader - Erratic Error"]
pub type ErrticerrR = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame Number of the Received SOF"]
pub type SoffnR = crate::FieldReader<u16>;
#[doc = "Field `DEVLNSTS` reader - Device Line Status"]
pub type DevlnstsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SuspstsR {
        SuspstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> EnumspdR {
        EnumspdR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errticerr(&self) -> ErrticerrR {
        ErrticerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SoffnR {
        SoffnR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23 - Device Line Status"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DevlnstsR {
        DevlnstsR::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "Device Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstsSpec;
impl crate::RegisterSpec for DstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DstsSpec {}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DstsSpec {
    const RESET_VALUE: u32 = 0x02;
}
