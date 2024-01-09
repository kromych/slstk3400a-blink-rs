#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DSTS_SPEC>;
#[doc = "Field `SUSPSTS` reader - Suspend Status"]
pub type SUSPSTS_R = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - Enumerated Speed"]
pub type ENUMSPD_R = crate::FieldReader<ENUMSPD_A>;
#[doc = "Enumerated Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENUMSPD_A {
    #[doc = "2: Low speed (PHY clock is running at 6 MHz)."]
    LS = 2,
    #[doc = "3: Full speed (PHY clock is running at 48 MHz)."]
    FS = 3,
}
impl From<ENUMSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENUMSPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ENUMSPD_A {
    type Ux = u8;
}
impl ENUMSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ENUMSPD_A> {
        match self.bits {
            2 => Some(ENUMSPD_A::LS),
            3 => Some(ENUMSPD_A::FS),
            _ => None,
        }
    }
    #[doc = "Low speed (PHY clock is running at 6 MHz)."]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == ENUMSPD_A::LS
    }
    #[doc = "Full speed (PHY clock is running at 48 MHz)."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == ENUMSPD_A::FS
    }
}
#[doc = "Field `ERRTICERR` reader - Erratic Error"]
pub type ERRTICERR_R = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame Number of the Received SOF"]
pub type SOFFN_R = crate::FieldReader<u16>;
#[doc = "Field `DEVLNSTS` reader - Device Line Status"]
pub type DEVLNSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errticerr(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23 - Device Line Status"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DSTS_SPEC {}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
