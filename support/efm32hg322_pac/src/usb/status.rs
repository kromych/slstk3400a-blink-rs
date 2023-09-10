#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `VREGOS` reader - VREGO Sense Output"]
pub type VREGOS_R = crate::BitReader;
#[doc = "Field `LEMACTIVE` reader - Low Energy Mode Active"]
pub type LEMACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VREGO Sense Output"]
    #[inline(always)]
    pub fn vregos(&self) -> VREGOS_R {
        VREGOS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LEMACTIVE_R {
        LEMACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "System Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
