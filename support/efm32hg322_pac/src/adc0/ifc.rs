#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Flag Clear"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Flag Clear"]
pub type ScanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Flag Clear"]
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Flag Clear"]
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn single(&mut self) -> SingleW<'_, IfcSpec> {
        SingleW::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn scan(&mut self) -> ScanW<'_, IfcSpec> {
        ScanW::new(self, 1)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SingleofW<'_, IfcSpec> {
        SingleofW::new(self, 8)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn scanof(&mut self) -> ScanofW<'_, IfcSpec> {
        ScanofW::new(self, 9)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
