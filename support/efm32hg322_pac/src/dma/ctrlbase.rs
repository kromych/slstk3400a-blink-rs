#[doc = "Register `CTRLBASE` reader"]
pub type R = crate::R<CtrlbaseSpec>;
#[doc = "Register `CTRLBASE` writer"]
pub type W = crate::W<CtrlbaseSpec>;
#[doc = "Field `CTRLBASE` reader - Channel Control Data Base Pointer"]
pub type CtrlbaseR = crate::FieldReader<u32>;
#[doc = "Field `CTRLBASE` writer - Channel Control Data Base Pointer"]
pub type CtrlbaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    pub fn ctrlbase(&self) -> CtrlbaseR {
        CtrlbaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Control Data Base Pointer"]
    #[inline(always)]
    pub fn ctrlbase(&mut self) -> CtrlbaseW<'_, CtrlbaseSpec> {
        CtrlbaseW::new(self, 0)
    }
}
#[doc = "Channel Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbaseSpec;
impl crate::RegisterSpec for CtrlbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlbase::R`](R) reader structure"]
impl crate::Readable for CtrlbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlbase::W`](W) writer structure"]
impl crate::Writable for CtrlbaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLBASE to value 0"]
impl crate::Resettable for CtrlbaseSpec {}
