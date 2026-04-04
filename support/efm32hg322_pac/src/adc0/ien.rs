#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Enable"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Enable"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Enable"]
pub type ScanR = crate::BitReader;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Enable"]
pub type ScanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEOF` reader - Single Result Overflow Interrupt Enable"]
pub type SingleofR = crate::BitReader;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Enable"]
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` reader - Scan Result Overflow Interrupt Enable"]
pub type ScanofR = crate::BitReader;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Enable"]
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&self) -> SingleofR {
        SingleofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&self) -> ScanofR {
        ScanofR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn single(&mut self) -> SingleW<'_, IenSpec> {
        SingleW::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&mut self) -> ScanW<'_, IenSpec> {
        ScanW::new(self, 1)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SingleofW<'_, IenSpec> {
        SingleofW::new(self, 8)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&mut self) -> ScanofW<'_, IenSpec> {
        ScanofW::new(self, 9)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
