#[doc = "Register `CHREQSTATUS` reader"]
pub type R = crate::R<ChreqstatusSpec>;
#[doc = "Field `CH0REQSTATUS` reader - Channel 0 Request Status"]
pub type Ch0reqstatusR = crate::BitReader;
#[doc = "Field `CH1REQSTATUS` reader - Channel 1 Request Status"]
pub type Ch1reqstatusR = crate::BitReader;
#[doc = "Field `CH2REQSTATUS` reader - Channel 2 Request Status"]
pub type Ch2reqstatusR = crate::BitReader;
#[doc = "Field `CH3REQSTATUS` reader - Channel 3 Request Status"]
pub type Ch3reqstatusR = crate::BitReader;
#[doc = "Field `CH4REQSTATUS` reader - Channel 4 Request Status"]
pub type Ch4reqstatusR = crate::BitReader;
#[doc = "Field `CH5REQSTATUS` reader - Channel 5 Request Status"]
pub type Ch5reqstatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Request Status"]
    #[inline(always)]
    pub fn ch0reqstatus(&self) -> Ch0reqstatusR {
        Ch0reqstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Request Status"]
    #[inline(always)]
    pub fn ch1reqstatus(&self) -> Ch1reqstatusR {
        Ch1reqstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Request Status"]
    #[inline(always)]
    pub fn ch2reqstatus(&self) -> Ch2reqstatusR {
        Ch2reqstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Request Status"]
    #[inline(always)]
    pub fn ch3reqstatus(&self) -> Ch3reqstatusR {
        Ch3reqstatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Request Status"]
    #[inline(always)]
    pub fn ch4reqstatus(&self) -> Ch4reqstatusR {
        Ch4reqstatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Request Status"]
    #[inline(always)]
    pub fn ch5reqstatus(&self) -> Ch5reqstatusR {
        Ch5reqstatusR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Channel Request Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chreqstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChreqstatusSpec;
impl crate::RegisterSpec for ChreqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chreqstatus::R`](R) reader structure"]
impl crate::Readable for ChreqstatusSpec {}
#[doc = "`reset()` method sets CHREQSTATUS to value 0"]
impl crate::Resettable for ChreqstatusSpec {}
