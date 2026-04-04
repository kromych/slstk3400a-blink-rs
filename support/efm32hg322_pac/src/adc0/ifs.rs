#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Flag Set"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Flag Set"]
pub type ScanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Flag Set"]
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Flag Set"]
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn single(&mut self) -> SingleW<'_, IfsSpec> {
        SingleW::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag Set"]
    #[inline(always)]
    pub fn scan(&mut self) -> ScanW<'_, IfsSpec> {
        ScanW::new(self, 1)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Flag Set"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SingleofW<'_, IfsSpec> {
        SingleofW::new(self, 8)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Flag Set"]
    #[inline(always)]
    pub fn scanof(&mut self) -> ScanofW<'_, IfsSpec> {
        ScanofW::new(self, 9)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
