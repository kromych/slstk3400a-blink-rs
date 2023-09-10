#[doc = "Register `TIMEBASE` reader"]
pub type R = crate::R<TIMEBASE_SPEC>;
#[doc = "Register `TIMEBASE` writer"]
pub type W = crate::W<TIMEBASE_SPEC>;
#[doc = "Field `BASE` reader - Timebase used by MSC to time flash writes and erases"]
pub type BASE_R = crate::FieldReader;
#[doc = "Field `BASE` writer - Timebase used by MSC to time flash writes and erases"]
pub type BASE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `PERIOD` reader - Sets the timebase period"]
pub type PERIOD_R = crate::BitReader;
#[doc = "Field `PERIOD` writer - Sets the timebase period"]
pub type PERIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<TIMEBASE_SPEC, 0> {
        BASE_W::new(self)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<TIMEBASE_SPEC, 16> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash Write and Erase Timebase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timebase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timebase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEBASE_SPEC;
impl crate::RegisterSpec for TIMEBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timebase::R`](R) reader structure"]
impl crate::Readable for TIMEBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timebase::W`](W) writer structure"]
impl crate::Writable for TIMEBASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEBASE to value 0x10"]
impl crate::Resettable for TIMEBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
