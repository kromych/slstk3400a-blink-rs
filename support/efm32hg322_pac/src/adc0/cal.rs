#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `SINGLEOFFSET` reader - Single Mode Offset Calibration Value"]
pub type SingleoffsetR = crate::FieldReader;
#[doc = "Field `SINGLEOFFSET` writer - Single Mode Offset Calibration Value"]
pub type SingleoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SINGLEGAIN` reader - Single Mode Gain Calibration Value"]
pub type SinglegainR = crate::FieldReader;
#[doc = "Field `SINGLEGAIN` writer - Single Mode Gain Calibration Value"]
pub type SinglegainW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SCANOFFSET` reader - Scan Mode Offset Calibration Value"]
pub type ScanoffsetR = crate::FieldReader;
#[doc = "Field `SCANOFFSET` writer - Scan Mode Offset Calibration Value"]
pub type ScanoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SCANGAIN` reader - Scan Mode Gain Calibration Value"]
pub type ScangainR = crate::FieldReader;
#[doc = "Field `SCANGAIN` writer - Scan Mode Gain Calibration Value"]
pub type ScangainW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Single Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn singleoffset(&self) -> SingleoffsetR {
        SingleoffsetR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&self) -> SinglegainR {
        SinglegainR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Scan Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn scanoffset(&self) -> ScanoffsetR {
        ScanoffsetR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&self) -> ScangainR {
        ScangainR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Single Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn singleoffset(&mut self) -> SingleoffsetW<'_, CalSpec> {
        SingleoffsetW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&mut self) -> SinglegainW<'_, CalSpec> {
        SinglegainW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Scan Mode Offset Calibration Value"]
    #[inline(always)]
    pub fn scanoffset(&mut self) -> ScanoffsetW<'_, CalSpec> {
        ScanoffsetW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&mut self) -> ScangainW<'_, CalSpec> {
        ScangainW::new(self, 24)
    }
}
#[doc = "Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0x3f00_3f00"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0x3f00_3f00;
}
