#[doc = "Register `CHREQSTATUS` reader"]
pub type R = crate::R<CHREQSTATUS_SPEC>;
#[doc = "Field `CH0REQSTATUS` reader - Channel 0 Request Status"]
pub type CH0REQSTATUS_R = crate::BitReader;
#[doc = "Field `CH1REQSTATUS` reader - Channel 1 Request Status"]
pub type CH1REQSTATUS_R = crate::BitReader;
#[doc = "Field `CH2REQSTATUS` reader - Channel 2 Request Status"]
pub type CH2REQSTATUS_R = crate::BitReader;
#[doc = "Field `CH3REQSTATUS` reader - Channel 3 Request Status"]
pub type CH3REQSTATUS_R = crate::BitReader;
#[doc = "Field `CH4REQSTATUS` reader - Channel 4 Request Status"]
pub type CH4REQSTATUS_R = crate::BitReader;
#[doc = "Field `CH5REQSTATUS` reader - Channel 5 Request Status"]
pub type CH5REQSTATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Request Status"]
    #[inline(always)]
    pub fn ch0reqstatus(&self) -> CH0REQSTATUS_R {
        CH0REQSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Request Status"]
    #[inline(always)]
    pub fn ch1reqstatus(&self) -> CH1REQSTATUS_R {
        CH1REQSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Request Status"]
    #[inline(always)]
    pub fn ch2reqstatus(&self) -> CH2REQSTATUS_R {
        CH2REQSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Request Status"]
    #[inline(always)]
    pub fn ch3reqstatus(&self) -> CH3REQSTATUS_R {
        CH3REQSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Request Status"]
    #[inline(always)]
    pub fn ch4reqstatus(&self) -> CH4REQSTATUS_R {
        CH4REQSTATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Request Status"]
    #[inline(always)]
    pub fn ch5reqstatus(&self) -> CH5REQSTATUS_R {
        CH5REQSTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Channel Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chreqstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHREQSTATUS_SPEC;
impl crate::RegisterSpec for CHREQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chreqstatus::R`](R) reader structure"]
impl crate::Readable for CHREQSTATUS_SPEC {}
#[doc = "`reset()` method sets CHREQSTATUS to value 0"]
impl crate::Resettable for CHREQSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
