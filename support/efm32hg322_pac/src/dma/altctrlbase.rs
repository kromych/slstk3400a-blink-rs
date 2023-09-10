#[doc = "Register `ALTCTRLBASE` reader"]
pub type R = crate::R<ALTCTRLBASE_SPEC>;
#[doc = "Field `ALTCTRLBASE` reader - Channel Alternate Control Data Base Pointer"]
pub type ALTCTRLBASE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Alternate Control Data Base Pointer"]
    #[inline(always)]
    pub fn altctrlbase(&self) -> ALTCTRLBASE_R {
        ALTCTRLBASE_R::new(self.bits)
    }
}
#[doc = "Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altctrlbase::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALTCTRLBASE_SPEC;
impl crate::RegisterSpec for ALTCTRLBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altctrlbase::R`](R) reader structure"]
impl crate::Readable for ALTCTRLBASE_SPEC {}
#[doc = "`reset()` method sets ALTCTRLBASE to value 0x80"]
impl crate::Resettable for ALTCTRLBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
