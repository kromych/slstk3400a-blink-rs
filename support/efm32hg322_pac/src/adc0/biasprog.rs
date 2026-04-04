#[doc = "Register `BIASPROG` reader"]
pub type R = crate::R<BiasprogSpec>;
#[doc = "Register `BIASPROG` writer"]
pub type W = crate::W<BiasprogSpec>;
#[doc = "Field `BIASPROG` reader - Bias Programming Value"]
pub type BiasprogR = crate::FieldReader;
#[doc = "Field `BIASPROG` writer - Bias Programming Value"]
pub type BiasprogW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HalfbiasR = crate::BitReader;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HalfbiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPBIAS` reader - Comparator Bias Value"]
pub type CompbiasR = crate::FieldReader;
#[doc = "Field `COMPBIAS` writer - Comparator Bias Value"]
pub type CompbiasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BiasprogR {
        BiasprogR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HalfbiasR {
        HalfbiasR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&self) -> CompbiasR {
        CompbiasR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BiasprogW<'_, BiasprogSpec> {
        BiasprogW::new(self, 0)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HalfbiasW<'_, BiasprogSpec> {
        HalfbiasW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&mut self) -> CompbiasW<'_, BiasprogSpec> {
        CompbiasW::new(self, 8)
    }
}
#[doc = "Bias Programming Register\n\nYou can [`read`](crate::Reg::read) this register and get [`biasprog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasprog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BiasprogSpec;
impl crate::RegisterSpec for BiasprogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasprog::R`](R) reader structure"]
impl crate::Readable for BiasprogSpec {}
#[doc = "`write(|w| ..)` method takes [`biasprog::W`](W) writer structure"]
impl crate::Writable for BiasprogSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIASPROG to value 0x0747"]
impl crate::Resettable for BiasprogSpec {
    const RESET_VALUE: u32 = 0x0747;
}
