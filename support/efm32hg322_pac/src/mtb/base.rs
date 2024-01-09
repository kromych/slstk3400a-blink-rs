#[doc = "Register `BASE` reader"]
pub type R = crate::R<BASE_SPEC>;
#[doc = "Register `BASE` writer"]
pub type W = crate::W<BASE_SPEC>;
#[doc = "Field `BASE` reader - The ram base address."]
pub type BASE_R = crate::FieldReader<u32>;
#[doc = "Field `BASE` writer - The ram base address."]
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ram base address."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<BASE_SPEC> {
        BASE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MTB Trace Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BASE_SPEC;
impl crate::RegisterSpec for BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base::R`](R) reader structure"]
impl crate::Readable for BASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`base::W`](W) writer structure"]
impl crate::Writable for BASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BASE to value 0x2000_0000"]
impl crate::Resettable for BASE_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
