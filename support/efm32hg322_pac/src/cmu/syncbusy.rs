#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSY_SPEC>;
#[doc = "Field `LFACLKEN0` reader - Low Frequency A Clock Enable 0 Busy"]
pub type LFACLKEN0_R = crate::BitReader;
#[doc = "Field `LFAPRESC0` reader - Low Frequency A Prescaler 0 Busy"]
pub type LFAPRESC0_R = crate::BitReader;
#[doc = "Field `LFBCLKEN0` reader - Low Frequency B Clock Enable 0 Busy"]
pub type LFBCLKEN0_R = crate::BitReader;
#[doc = "Field `LFBPRESC0` reader - Low Frequency B Prescaler 0 Busy"]
pub type LFBPRESC0_R = crate::BitReader;
#[doc = "Field `LFCCLKEN0` reader - Low Frequency C Clock Enable 0 Busy"]
pub type LFCCLKEN0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Low Frequency A Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfaclken0(&self) -> LFACLKEN0_R {
        LFACLKEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Frequency A Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfapresc0(&self) -> LFAPRESC0_R {
        LFAPRESC0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Frequency B Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfbclken0(&self) -> LFBCLKEN0_R {
        LFBCLKEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Frequency B Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfbpresc0(&self) -> LFBPRESC0_R {
        LFBPRESC0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Frequency C Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfcclken0(&self) -> LFCCLKEN0_R {
        LFCCLKEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
