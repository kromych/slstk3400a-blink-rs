#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `RUNNING` reader - Running"]
pub type RUNNING_R = crate::BitReader;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `TOPBV` reader - TOPB Valid"]
pub type TOPBV_R = crate::BitReader;
#[doc = "Field `CCVBV0` reader - CC0 CCVB Valid"]
pub type CCVBV0_R = crate::BitReader;
#[doc = "Field `CCVBV1` reader - CC1 CCVB Valid"]
pub type CCVBV1_R = crate::BitReader;
#[doc = "Field `CCVBV2` reader - CC2 CCVB Valid"]
pub type CCVBV2_R = crate::BitReader;
#[doc = "Field `ICV0` reader - CC0 Input Capture Valid"]
pub type ICV0_R = crate::BitReader;
#[doc = "Field `ICV1` reader - CC1 Input Capture Valid"]
pub type ICV1_R = crate::BitReader;
#[doc = "Field `ICV2` reader - CC2 Input Capture Valid"]
pub type ICV2_R = crate::BitReader;
#[doc = "Field `CCPOL0` reader - CC0 Polarity"]
pub type CCPOL0_R = crate::BitReader;
#[doc = "Field `CCPOL1` reader - CC1 Polarity"]
pub type CCPOL1_R = crate::BitReader;
#[doc = "Field `CCPOL2` reader - CC2 Polarity"]
pub type CCPOL2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOPB Valid"]
    #[inline(always)]
    pub fn topbv(&self) -> TOPBV_R {
        TOPBV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CC0 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv0(&self) -> CCVBV0_R {
        CCVBV0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv1(&self) -> CCVBV1_R {
        CCVBV1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv2(&self) -> CCVBV2_R {
        CCVBV2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - CC0 Input Capture Valid"]
    #[inline(always)]
    pub fn icv0(&self) -> ICV0_R {
        ICV0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC1 Input Capture Valid"]
    #[inline(always)]
    pub fn icv1(&self) -> ICV1_R {
        ICV1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CC2 Input Capture Valid"]
    #[inline(always)]
    pub fn icv2(&self) -> ICV2_R {
        ICV2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - CC0 Polarity"]
    #[inline(always)]
    pub fn ccpol0(&self) -> CCPOL0_R {
        CCPOL0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CC1 Polarity"]
    #[inline(always)]
    pub fn ccpol1(&self) -> CCPOL1_R {
        CCPOL1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CC2 Polarity"]
    #[inline(always)]
    pub fn ccpol2(&self) -> CCPOL2_R {
        CCPOL2_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
