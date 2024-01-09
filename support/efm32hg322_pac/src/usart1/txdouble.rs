#[doc = "Register `TXDOUBLE` writer"]
pub type W = crate::W<TXDOUBLE_SPEC>;
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type TXDATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type TXDATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata0(&mut self) -> TXDATA0_W<TXDOUBLE_SPEC> {
        TXDATA0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata1(&mut self) -> TXDATA1_W<TXDOUBLE_SPEC> {
        TXDATA1_W::new(self, 8)
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
#[doc = "TX Buffer Double Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdouble::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDOUBLE_SPEC;
impl crate::RegisterSpec for TXDOUBLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdouble::W`](W) writer structure"]
impl crate::Writable for TXDOUBLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDOUBLE to value 0"]
impl crate::Resettable for TXDOUBLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
