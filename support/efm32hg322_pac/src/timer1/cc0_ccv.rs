#[doc = "Register `CC0_CCV` reader"]
pub type R = crate::R<Cc0CcvSpec>;
#[doc = "Register `CC0_CCV` writer"]
pub type W = crate::W<Cc0CcvSpec>;
#[doc = "Field `CCV` reader - CC Channel Value"]
pub type CcvR = crate::FieldReader<u16>;
#[doc = "Field `CCV` writer - CC Channel Value"]
pub type CcvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CcvR {
        CcvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&mut self) -> CcvW<'_, Cc0CcvSpec> {
        CcvW::new(self, 0)
    }
}
#[doc = "CC Channel Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ccv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0CcvSpec;
impl crate::RegisterSpec for Cc0CcvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ccv::R`](R) reader structure"]
impl crate::Readable for Cc0CcvSpec {}
#[doc = "`write(|w| ..)` method takes [`cc0_ccv::W`](W) writer structure"]
impl crate::Writable for Cc0CcvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC0_CCV to value 0"]
impl crate::Resettable for Cc0CcvSpec {}
