#[doc = "Register `CC0_CCVB` reader"]
pub type R = crate::R<Cc0CcvbSpec>;
#[doc = "Register `CC0_CCVB` writer"]
pub type W = crate::W<Cc0CcvbSpec>;
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub type CcvbR = crate::FieldReader<u16>;
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub type CcvbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CcvbR {
        CcvbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&mut self) -> CcvbW<'_, Cc0CcvbSpec> {
        CcvbW::new(self, 0)
    }
}
#[doc = "CC Channel Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccvb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ccvb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0CcvbSpec;
impl crate::RegisterSpec for Cc0CcvbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ccvb::R`](R) reader structure"]
impl crate::Readable for Cc0CcvbSpec {}
#[doc = "`write(|w| ..)` method takes [`cc0_ccvb::W`](W) writer structure"]
impl crate::Writable for Cc0CcvbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC0_CCVB to value 0"]
impl crate::Resettable for Cc0CcvbSpec {}
