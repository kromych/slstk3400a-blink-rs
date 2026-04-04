#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RUNNING` reader - Running"]
pub type RunningR = crate::BitReader;
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `TOPBV` reader - TOPB Valid"]
pub type TopbvR = crate::BitReader;
#[doc = "Field `CCVBV0` reader - CC0 CCVB Valid"]
pub type Ccvbv0R = crate::BitReader;
#[doc = "Field `CCVBV1` reader - CC1 CCVB Valid"]
pub type Ccvbv1R = crate::BitReader;
#[doc = "Field `CCVBV2` reader - CC2 CCVB Valid"]
pub type Ccvbv2R = crate::BitReader;
#[doc = "Field `ICV0` reader - CC0 Input Capture Valid"]
pub type Icv0R = crate::BitReader;
#[doc = "Field `ICV1` reader - CC1 Input Capture Valid"]
pub type Icv1R = crate::BitReader;
#[doc = "Field `ICV2` reader - CC2 Input Capture Valid"]
pub type Icv2R = crate::BitReader;
#[doc = "Field `CCPOL0` reader - CC0 Polarity"]
pub type Ccpol0R = crate::BitReader;
#[doc = "Field `CCPOL1` reader - CC1 Polarity"]
pub type Ccpol1R = crate::BitReader;
#[doc = "Field `CCPOL2` reader - CC2 Polarity"]
pub type Ccpol2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Running"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOPB Valid"]
    #[inline(always)]
    pub fn topbv(&self) -> TopbvR {
        TopbvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CC0 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv0(&self) -> Ccvbv0R {
        Ccvbv0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv1(&self) -> Ccvbv1R {
        Ccvbv1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv2(&self) -> Ccvbv2R {
        Ccvbv2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - CC0 Input Capture Valid"]
    #[inline(always)]
    pub fn icv0(&self) -> Icv0R {
        Icv0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC1 Input Capture Valid"]
    #[inline(always)]
    pub fn icv1(&self) -> Icv1R {
        Icv1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CC2 Input Capture Valid"]
    #[inline(always)]
    pub fn icv2(&self) -> Icv2R {
        Icv2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - CC0 Polarity"]
    #[inline(always)]
    pub fn ccpol0(&self) -> Ccpol0R {
        Ccpol0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CC1 Polarity"]
    #[inline(always)]
    pub fn ccpol1(&self) -> Ccpol1R {
        Ccpol1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CC2 Polarity"]
    #[inline(always)]
    pub fn ccpol2(&self) -> Ccpol2R {
        Ccpol2R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
