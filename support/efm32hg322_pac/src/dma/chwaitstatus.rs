#[doc = "Register `CHWAITSTATUS` reader"]
pub type R = crate::R<ChwaitstatusSpec>;
#[doc = "Field `CH0WAITSTATUS` reader - Channel 0 Wait on Request Status"]
pub type Ch0waitstatusR = crate::BitReader;
#[doc = "Field `CH1WAITSTATUS` reader - Channel 1 Wait on Request Status"]
pub type Ch1waitstatusR = crate::BitReader;
#[doc = "Field `CH2WAITSTATUS` reader - Channel 2 Wait on Request Status"]
pub type Ch2waitstatusR = crate::BitReader;
#[doc = "Field `CH3WAITSTATUS` reader - Channel 3 Wait on Request Status"]
pub type Ch3waitstatusR = crate::BitReader;
#[doc = "Field `CH4WAITSTATUS` reader - Channel 4 Wait on Request Status"]
pub type Ch4waitstatusR = crate::BitReader;
#[doc = "Field `CH5WAITSTATUS` reader - Channel 5 Wait on Request Status"]
pub type Ch5waitstatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Wait on Request Status"]
    #[inline(always)]
    pub fn ch0waitstatus(&self) -> Ch0waitstatusR {
        Ch0waitstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Wait on Request Status"]
    #[inline(always)]
    pub fn ch1waitstatus(&self) -> Ch1waitstatusR {
        Ch1waitstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Wait on Request Status"]
    #[inline(always)]
    pub fn ch2waitstatus(&self) -> Ch2waitstatusR {
        Ch2waitstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Wait on Request Status"]
    #[inline(always)]
    pub fn ch3waitstatus(&self) -> Ch3waitstatusR {
        Ch3waitstatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Wait on Request Status"]
    #[inline(always)]
    pub fn ch4waitstatus(&self) -> Ch4waitstatusR {
        Ch4waitstatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Wait on Request Status"]
    #[inline(always)]
    pub fn ch5waitstatus(&self) -> Ch5waitstatusR {
        Ch5waitstatusR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Channel Wait on Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chwaitstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChwaitstatusSpec;
impl crate::RegisterSpec for ChwaitstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chwaitstatus::R`](R) reader structure"]
impl crate::Readable for ChwaitstatusSpec {}
#[doc = "`reset()` method sets CHWAITSTATUS to value 0x3f"]
impl crate::Resettable for ChwaitstatusSpec {
    const RESET_VALUE: u32 = 0x3f;
}
