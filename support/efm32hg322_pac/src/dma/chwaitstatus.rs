#[doc = "Register `CHWAITSTATUS` reader"]
pub type R = crate::R<CHWAITSTATUS_SPEC>;
#[doc = "Field `CH0WAITSTATUS` reader - Channel 0 Wait on Request Status"]
pub type CH0WAITSTATUS_R = crate::BitReader;
#[doc = "Field `CH1WAITSTATUS` reader - Channel 1 Wait on Request Status"]
pub type CH1WAITSTATUS_R = crate::BitReader;
#[doc = "Field `CH2WAITSTATUS` reader - Channel 2 Wait on Request Status"]
pub type CH2WAITSTATUS_R = crate::BitReader;
#[doc = "Field `CH3WAITSTATUS` reader - Channel 3 Wait on Request Status"]
pub type CH3WAITSTATUS_R = crate::BitReader;
#[doc = "Field `CH4WAITSTATUS` reader - Channel 4 Wait on Request Status"]
pub type CH4WAITSTATUS_R = crate::BitReader;
#[doc = "Field `CH5WAITSTATUS` reader - Channel 5 Wait on Request Status"]
pub type CH5WAITSTATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Wait on Request Status"]
    #[inline(always)]
    pub fn ch0waitstatus(&self) -> CH0WAITSTATUS_R {
        CH0WAITSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Wait on Request Status"]
    #[inline(always)]
    pub fn ch1waitstatus(&self) -> CH1WAITSTATUS_R {
        CH1WAITSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Wait on Request Status"]
    #[inline(always)]
    pub fn ch2waitstatus(&self) -> CH2WAITSTATUS_R {
        CH2WAITSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Wait on Request Status"]
    #[inline(always)]
    pub fn ch3waitstatus(&self) -> CH3WAITSTATUS_R {
        CH3WAITSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Wait on Request Status"]
    #[inline(always)]
    pub fn ch4waitstatus(&self) -> CH4WAITSTATUS_R {
        CH4WAITSTATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Wait on Request Status"]
    #[inline(always)]
    pub fn ch5waitstatus(&self) -> CH5WAITSTATUS_R {
        CH5WAITSTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Channel Wait on Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chwaitstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHWAITSTATUS_SPEC;
impl crate::RegisterSpec for CHWAITSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chwaitstatus::R`](R) reader structure"]
impl crate::Readable for CHWAITSTATUS_SPEC {}
#[doc = "`reset()` method sets CHWAITSTATUS to value 0x3f"]
impl crate::Resettable for CHWAITSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
