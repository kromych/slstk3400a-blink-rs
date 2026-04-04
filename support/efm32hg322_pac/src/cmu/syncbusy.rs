#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `LFACLKEN0` reader - Low Frequency A Clock Enable 0 Busy"]
pub type Lfaclken0R = crate::BitReader;
#[doc = "Field `LFAPRESC0` reader - Low Frequency A Prescaler 0 Busy"]
pub type Lfapresc0R = crate::BitReader;
#[doc = "Field `LFBCLKEN0` reader - Low Frequency B Clock Enable 0 Busy"]
pub type Lfbclken0R = crate::BitReader;
#[doc = "Field `LFBPRESC0` reader - Low Frequency B Prescaler 0 Busy"]
pub type Lfbpresc0R = crate::BitReader;
#[doc = "Field `LFCCLKEN0` reader - Low Frequency C Clock Enable 0 Busy"]
pub type Lfcclken0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Low Frequency A Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfaclken0(&self) -> Lfaclken0R {
        Lfaclken0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Frequency A Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfapresc0(&self) -> Lfapresc0R {
        Lfapresc0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Frequency B Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfbclken0(&self) -> Lfbclken0R {
        Lfbclken0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Frequency B Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfbpresc0(&self) -> Lfbpresc0R {
        Lfbpresc0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Frequency C Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfcclken0(&self) -> Lfcclken0R {
        Lfcclken0R::new(((self.bits >> 8) & 1) != 0)
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
