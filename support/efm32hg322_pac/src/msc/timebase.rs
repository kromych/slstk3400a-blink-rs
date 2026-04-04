#[doc = "Register `TIMEBASE` reader"]
pub type R = crate::R<TimebaseSpec>;
#[doc = "Register `TIMEBASE` writer"]
pub type W = crate::W<TimebaseSpec>;
#[doc = "Field `BASE` reader - Timebase used by MSC to time flash writes and erases"]
pub type BaseR = crate::FieldReader;
#[doc = "Field `BASE` writer - Timebase used by MSC to time flash writes and erases"]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PERIOD` reader - Sets the timebase period"]
pub type PeriodR = crate::BitReader;
#[doc = "Field `PERIOD` writer - Sets the timebase period"]
pub type PeriodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&mut self) -> BaseW<'_, TimebaseSpec> {
        BaseW::new(self, 0)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, TimebaseSpec> {
        PeriodW::new(self, 16)
    }
}
#[doc = "Flash Write and Erase Timebase\n\nYou can [`read`](crate::Reg::read) this register and get [`timebase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timebase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimebaseSpec;
impl crate::RegisterSpec for TimebaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timebase::R`](R) reader structure"]
impl crate::Readable for TimebaseSpec {}
#[doc = "`write(|w| ..)` method takes [`timebase::W`](W) writer structure"]
impl crate::Writable for TimebaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEBASE to value 0x10"]
impl crate::Resettable for TimebaseSpec {
    const RESET_VALUE: u32 = 0x10;
}
