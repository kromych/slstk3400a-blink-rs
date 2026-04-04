#[doc = "Register `TOP` reader"]
pub type R = crate::R<TopSpec>;
#[doc = "Field `TOP` reader - Counter Top Value"]
pub type TopR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Top Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopSpec;
impl crate::RegisterSpec for TopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TopSpec {}
#[doc = "`reset()` method sets TOP to value 0xff"]
impl crate::Resettable for TopSpec {
    const RESET_VALUE: u32 = 0xff;
}
