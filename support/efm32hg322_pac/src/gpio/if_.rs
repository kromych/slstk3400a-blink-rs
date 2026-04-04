#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `EXT` reader - External Interrupt Flag n"]
pub type ExtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt Flag n"]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
