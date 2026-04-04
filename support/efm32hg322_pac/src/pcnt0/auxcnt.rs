#[doc = "Register `AUXCNT` reader"]
pub type R = crate::R<AuxcntSpec>;
#[doc = "Register `AUXCNT` writer"]
pub type W = crate::W<AuxcntSpec>;
#[doc = "Field `AUXCNT` reader - Auxiliary Counter Value"]
pub type AuxcntR = crate::FieldReader<u16>;
#[doc = "Field `AUXCNT` writer - Auxiliary Counter Value"]
pub type AuxcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AuxcntR {
        AuxcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&mut self) -> AuxcntW<'_, AuxcntSpec> {
        AuxcntW::new(self, 0)
    }
}
#[doc = "Auxiliary Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`auxcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auxcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxcntSpec;
impl crate::RegisterSpec for AuxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxcnt::R`](R) reader structure"]
impl crate::Readable for AuxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`auxcnt::W`](W) writer structure"]
impl crate::Writable for AuxcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUXCNT to value 0"]
impl crate::Resettable for AuxcntSpec {}
