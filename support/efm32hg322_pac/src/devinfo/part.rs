#[doc = "Register `PART` reader"]
pub type R = crate::R<PartSpec>;
#[doc = "Field `PART_NUMBER` reader - Device part number"]
pub type PartNumberR = crate::FieldReader<u16>;
#[doc = "Field `DEVICE_FAMILY` reader - Device Family, 0x47 for Gecko"]
pub type DeviceFamilyR = crate::FieldReader;
#[doc = "Field `PROD_REV` reader - Production revision"]
pub type ProdRevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Device part number"]
    #[inline(always)]
    pub fn part_number(&self) -> PartNumberR {
        PartNumberR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Device Family, 0x47 for Gecko"]
    #[inline(always)]
    pub fn device_family(&self) -> DeviceFamilyR {
        DeviceFamilyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Production revision"]
    #[inline(always)]
    pub fn prod_rev(&self) -> ProdRevR {
        ProdRevR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Part description\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartSpec;
impl crate::RegisterSpec for PartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`part::R`](R) reader structure"]
impl crate::Readable for PartSpec {}
