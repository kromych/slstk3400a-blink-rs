#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ACMPACT` reader - Analog Comparator Active"]
pub type AcmpactR = crate::BitReader;
#[doc = "Field `ACMPOUT` reader - Analog Comparator Output"]
pub type AcmpoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog Comparator Active"]
    #[inline(always)]
    pub fn acmpact(&self) -> AcmpactR {
        AcmpactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Output"]
    #[inline(always)]
    pub fn acmpout(&self) -> AcmpoutR {
        AcmpoutR::new(((self.bits >> 1) & 1) != 0)
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
