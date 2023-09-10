#[doc = "Register `CC1_CCVP` reader"]
pub type R = crate::R<CC1_CCVP_SPEC>;
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CCVP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ccvp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC1_CCVP_SPEC;
impl crate::RegisterSpec for CC1_CCVP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ccvp::R`](R) reader structure"]
impl crate::Readable for CC1_CCVP_SPEC {}
#[doc = "`reset()` method sets CC1_CCVP to value 0"]
impl crate::Resettable for CC1_CCVP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
