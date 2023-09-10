#[doc = "Register `CHSREQSTATUS` reader"]
pub type R = crate::R<CHSREQSTATUS_SPEC>;
#[doc = "Field `CH0SREQSTATUS` reader - Channel 0 Single Request Status"]
pub type CH0SREQSTATUS_R = crate::BitReader;
#[doc = "Field `CH1SREQSTATUS` reader - Channel 1 Single Request Status"]
pub type CH1SREQSTATUS_R = crate::BitReader;
#[doc = "Field `CH2SREQSTATUS` reader - Channel 2 Single Request Status"]
pub type CH2SREQSTATUS_R = crate::BitReader;
#[doc = "Field `CH3SREQSTATUS` reader - Channel 3 Single Request Status"]
pub type CH3SREQSTATUS_R = crate::BitReader;
#[doc = "Field `CH4SREQSTATUS` reader - Channel 4 Single Request Status"]
pub type CH4SREQSTATUS_R = crate::BitReader;
#[doc = "Field `CH5SREQSTATUS` reader - Channel 5 Single Request Status"]
pub type CH5SREQSTATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Single Request Status"]
    #[inline(always)]
    pub fn ch0sreqstatus(&self) -> CH0SREQSTATUS_R {
        CH0SREQSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Single Request Status"]
    #[inline(always)]
    pub fn ch1sreqstatus(&self) -> CH1SREQSTATUS_R {
        CH1SREQSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Single Request Status"]
    #[inline(always)]
    pub fn ch2sreqstatus(&self) -> CH2SREQSTATUS_R {
        CH2SREQSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Single Request Status"]
    #[inline(always)]
    pub fn ch3sreqstatus(&self) -> CH3SREQSTATUS_R {
        CH3SREQSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Single Request Status"]
    #[inline(always)]
    pub fn ch4sreqstatus(&self) -> CH4SREQSTATUS_R {
        CH4SREQSTATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Single Request Status"]
    #[inline(always)]
    pub fn ch5sreqstatus(&self) -> CH5SREQSTATUS_R {
        CH5SREQSTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Channel Single Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsreqstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSREQSTATUS_SPEC;
impl crate::RegisterSpec for CHSREQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsreqstatus::R`](R) reader structure"]
impl crate::Readable for CHSREQSTATUS_SPEC {}
#[doc = "`reset()` method sets CHSREQSTATUS to value 0"]
impl crate::Resettable for CHSREQSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
