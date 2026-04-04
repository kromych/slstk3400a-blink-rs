#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Flag"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Flag"]
pub type ScanR = crate::BitReader;
#[doc = "Field `SINGLEOF` reader - Single Result Overflow Interrupt Flag"]
pub type SingleofR = crate::BitReader;
#[doc = "Field `SCANOF` reader - Scan Result Overflow Interrupt Flag"]
pub type ScanofR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&self) -> SingleofR {
        SingleofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&self) -> ScanofR {
        ScanofR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
