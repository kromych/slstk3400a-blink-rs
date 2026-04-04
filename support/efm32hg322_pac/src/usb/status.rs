#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `VREGOS` reader - VREGO Sense Output"]
pub type VregosR = crate::BitReader;
#[doc = "Field `LEMACTIVE` reader - Low Energy Mode Active"]
pub type LemactiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VREGO Sense Output"]
    #[inline(always)]
    pub fn vregos(&self) -> VregosR {
        VregosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LemactiveR {
        LemactiveR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
