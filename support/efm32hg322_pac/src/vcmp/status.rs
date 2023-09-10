#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `VCMPACT` reader - Voltage Supply Comparator Active"]
pub type VCMPACT_R = crate::BitReader;
#[doc = "Field `VCMPOUT` reader - Voltage Supply Comparator Output"]
pub type VCMPOUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Active"]
    #[inline(always)]
    pub fn vcmpact(&self) -> VCMPACT_R {
        VCMPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Supply Comparator Output"]
    #[inline(always)]
    pub fn vcmpout(&self) -> VCMPOUT_R {
        VCMPOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
