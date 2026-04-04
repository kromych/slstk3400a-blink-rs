#[doc = "Register `LFBCLKEN0` reader"]
pub type R = crate::R<Lfbclken0Spec>;
#[doc = "Register `LFBCLKEN0` writer"]
pub type W = crate::W<Lfbclken0Spec>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Clock Enable"]
pub type Leuart0R = crate::BitReader;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Clock Enable"]
pub type Leuart0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&self) -> Leuart0R {
        Leuart0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> Leuart0W<'_, Lfbclken0Spec> {
        Leuart0W::new(self, 0)
    }
}
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfbclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfbclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfbclken0Spec;
impl crate::RegisterSpec for Lfbclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbclken0::R`](R) reader structure"]
impl crate::Readable for Lfbclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfbclken0::W`](W) writer structure"]
impl crate::Writable for Lfbclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFBCLKEN0 to value 0"]
impl crate::Resettable for Lfbclken0Spec {}
