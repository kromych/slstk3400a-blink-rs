#[doc = "Register `ERRORC` reader"]
pub type R = crate::R<ErrorcSpec>;
#[doc = "Register `ERRORC` writer"]
pub type W = crate::W<ErrorcSpec>;
#[doc = "Field `ERRORC` reader - Bus Error Clear"]
pub type ErrorcR = crate::BitReader;
#[doc = "Field `ERRORC` writer - Bus Error Clear"]
pub type ErrorcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bus Error Clear"]
    #[inline(always)]
    pub fn errorc(&self) -> ErrorcR {
        ErrorcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Clear"]
    #[inline(always)]
    pub fn errorc(&mut self) -> ErrorcW<'_, ErrorcSpec> {
        ErrorcW::new(self, 0)
    }
}
#[doc = "Bus Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errorc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorcSpec;
impl crate::RegisterSpec for ErrorcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorc::R`](R) reader structure"]
impl crate::Readable for ErrorcSpec {}
#[doc = "`write(|w| ..)` method takes [`errorc::W`](W) writer structure"]
impl crate::Writable for ErrorcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRORC to value 0"]
impl crate::Resettable for ErrorcSpec {}
