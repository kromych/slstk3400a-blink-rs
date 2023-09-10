#[doc = "Register `IRQLATENCY` reader"]
pub type R = crate::R<IRQLATENCY_SPEC>;
#[doc = "Register `IRQLATENCY` writer"]
pub type W = crate::W<IRQLATENCY_SPEC>;
#[doc = "Field `IRQLATENCY` reader - Irq Latency Register"]
pub type IRQLATENCY_R = crate::FieldReader;
#[doc = "Field `IRQLATENCY` writer - Irq Latency Register"]
pub type IRQLATENCY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    pub fn irqlatency(&self) -> IRQLATENCY_R {
        IRQLATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    #[must_use]
    pub fn irqlatency(&mut self) -> IRQLATENCY_W<IRQLATENCY_SPEC, 0> {
        IRQLATENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Irq Latency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqlatency::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqlatency::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQLATENCY_SPEC;
impl crate::RegisterSpec for IRQLATENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqlatency::R`](R) reader structure"]
impl crate::Readable for IRQLATENCY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irqlatency::W`](W) writer structure"]
impl crate::Writable for IRQLATENCY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQLATENCY to value 0"]
impl crate::Resettable for IRQLATENCY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
