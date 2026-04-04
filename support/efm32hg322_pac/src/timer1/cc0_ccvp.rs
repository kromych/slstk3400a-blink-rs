#[doc = "Register `CC0_CCVP` reader"]
pub type R = crate::R<Cc0CcvpSpec>;
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CcvpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CcvpR {
        CcvpR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CC Channel Value Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccvp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0CcvpSpec;
impl crate::RegisterSpec for Cc0CcvpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ccvp::R`](R) reader structure"]
impl crate::Readable for Cc0CcvpSpec {}
#[doc = "`reset()` method sets CC0_CCVP to value 0"]
impl crate::Resettable for Cc0CcvpSpec {}
