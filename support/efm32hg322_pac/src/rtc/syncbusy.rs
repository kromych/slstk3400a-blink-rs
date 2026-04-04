#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CtrlR = crate::BitReader;
#[doc = "Field `COMP0` reader - COMP0 Register Busy"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP1` reader - COMP1 Register Busy"]
pub type Comp1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMP0 Register Busy"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - COMP1 Register Busy"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
