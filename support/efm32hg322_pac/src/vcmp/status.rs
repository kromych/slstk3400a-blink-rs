#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `VCMPACT` reader - Voltage Supply Comparator Active"]
pub type VcmpactR = crate::BitReader;
#[doc = "Field `VCMPOUT` reader - Voltage Supply Comparator Output"]
pub type VcmpoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Active"]
    #[inline(always)]
    pub fn vcmpact(&self) -> VcmpactR {
        VcmpactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Supply Comparator Output"]
    #[inline(always)]
    pub fn vcmpout(&self) -> VcmpoutR {
        VcmpoutR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
