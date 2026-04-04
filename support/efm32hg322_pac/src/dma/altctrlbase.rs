#[doc = "Register `ALTCTRLBASE` reader"]
pub type R = crate::R<AltctrlbaseSpec>;
#[doc = "Field `ALTCTRLBASE` reader - Channel Alternate Control Data Base Pointer"]
pub type AltctrlbaseR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Alternate Control Data Base Pointer"]
    #[inline(always)]
    pub fn altctrlbase(&self) -> AltctrlbaseR {
        AltctrlbaseR::new(self.bits)
    }
}
#[doc = "Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altctrlbase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltctrlbaseSpec;
impl crate::RegisterSpec for AltctrlbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altctrlbase::R`](R) reader structure"]
impl crate::Readable for AltctrlbaseSpec {}
#[doc = "`reset()` method sets ALTCTRLBASE to value 0x80"]
impl crate::Resettable for AltctrlbaseSpec {
    const RESET_VALUE: u32 = 0x80;
}
