#[doc = "Register `CHSREQSTATUS` reader"]
pub type R = crate::R<ChsreqstatusSpec>;
#[doc = "Field `CH0SREQSTATUS` reader - Channel 0 Single Request Status"]
pub type Ch0sreqstatusR = crate::BitReader;
#[doc = "Field `CH1SREQSTATUS` reader - Channel 1 Single Request Status"]
pub type Ch1sreqstatusR = crate::BitReader;
#[doc = "Field `CH2SREQSTATUS` reader - Channel 2 Single Request Status"]
pub type Ch2sreqstatusR = crate::BitReader;
#[doc = "Field `CH3SREQSTATUS` reader - Channel 3 Single Request Status"]
pub type Ch3sreqstatusR = crate::BitReader;
#[doc = "Field `CH4SREQSTATUS` reader - Channel 4 Single Request Status"]
pub type Ch4sreqstatusR = crate::BitReader;
#[doc = "Field `CH5SREQSTATUS` reader - Channel 5 Single Request Status"]
pub type Ch5sreqstatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Single Request Status"]
    #[inline(always)]
    pub fn ch0sreqstatus(&self) -> Ch0sreqstatusR {
        Ch0sreqstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Single Request Status"]
    #[inline(always)]
    pub fn ch1sreqstatus(&self) -> Ch1sreqstatusR {
        Ch1sreqstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Single Request Status"]
    #[inline(always)]
    pub fn ch2sreqstatus(&self) -> Ch2sreqstatusR {
        Ch2sreqstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Single Request Status"]
    #[inline(always)]
    pub fn ch3sreqstatus(&self) -> Ch3sreqstatusR {
        Ch3sreqstatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Single Request Status"]
    #[inline(always)]
    pub fn ch4sreqstatus(&self) -> Ch4sreqstatusR {
        Ch4sreqstatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Single Request Status"]
    #[inline(always)]
    pub fn ch5sreqstatus(&self) -> Ch5sreqstatusR {
        Ch5sreqstatusR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Channel Single Request Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chsreqstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChsreqstatusSpec;
impl crate::RegisterSpec for ChsreqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsreqstatus::R`](R) reader structure"]
impl crate::Readable for ChsreqstatusSpec {}
#[doc = "`reset()` method sets CHSREQSTATUS to value 0"]
impl crate::Resettable for ChsreqstatusSpec {}
