#[doc = "Register `IDAC0CAL0` reader"]
pub type R = crate::R<Idac0cal0Spec>;
#[doc = "Field `RANGE0` reader - Current range 0 tuning value for IDAC0"]
pub type Range0R = crate::FieldReader;
#[doc = "Field `RANGE1` reader - Current range 1 tuning value for IDAC0"]
pub type Range1R = crate::FieldReader;
#[doc = "Field `RANGE2` reader - Current range 2 tuning value for IDAC0"]
pub type Range2R = crate::FieldReader;
#[doc = "Field `RANGE3` reader - Current range 3 tuning value for IDAC0"]
pub type Range3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current range 0 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range0(&self) -> Range0R {
        Range0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Current range 1 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range1(&self) -> Range1R {
        Range1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current range 2 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range2(&self) -> Range2R {
        Range2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Current range 3 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range3(&self) -> Range3R {
        Range3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "IDAC0 calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`idac0cal0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idac0cal0Spec;
impl crate::RegisterSpec for Idac0cal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idac0cal0::R`](R) reader structure"]
impl crate::Readable for Idac0cal0Spec {}
