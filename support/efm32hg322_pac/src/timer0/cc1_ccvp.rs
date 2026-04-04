#[doc = "Register `CC1_CCVP` reader"]
pub type R = crate::R<Cc1CcvpSpec>;
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CcvpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CcvpR {
        CcvpR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CC Channel Value Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ccvp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc1CcvpSpec;
impl crate::RegisterSpec for Cc1CcvpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ccvp::R`](R) reader structure"]
impl crate::Readable for Cc1CcvpSpec {}
#[doc = "`reset()` method sets CC1_CCVP to value 0"]
impl crate::Resettable for Cc1CcvpSpec {}
