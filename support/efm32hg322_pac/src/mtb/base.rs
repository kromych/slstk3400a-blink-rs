#[doc = "Register `BASE` reader"]
pub type R = crate::R<BaseSpec>;
#[doc = "Register `BASE` writer"]
pub type W = crate::W<BaseSpec>;
#[doc = "Field `BASE` reader - The ram base address."]
pub type BaseR = crate::FieldReader<u32>;
#[doc = "Field `BASE` writer - The ram base address."]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    pub fn base(&mut self) -> BaseW<'_, BaseSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "MTB Trace Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseSpec;
impl crate::RegisterSpec for BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base::R`](R) reader structure"]
impl crate::Readable for BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`base::W`](W) writer structure"]
impl crate::Writable for BaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BASE to value 0x2000_0000"]
impl crate::Resettable for BaseSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
