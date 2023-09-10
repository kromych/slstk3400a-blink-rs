#[doc = "Register `PART` reader"]
pub type R = crate::R<PART_SPEC>;
#[doc = "Field `PART_NUMBER` reader - Device part number"]
pub type PART_NUMBER_R = crate::FieldReader<u16>;
#[doc = "Field `DEVICE_FAMILY` reader - Device Family, 0x47 for Gecko"]
pub type DEVICE_FAMILY_R = crate::FieldReader;
#[doc = "Field `PROD_REV` reader - Production revision"]
pub type PROD_REV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Device part number"]
    #[inline(always)]
    pub fn part_number(&self) -> PART_NUMBER_R {
        PART_NUMBER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Device Family, 0x47 for Gecko"]
    #[inline(always)]
    pub fn device_family(&self) -> DEVICE_FAMILY_R {
        DEVICE_FAMILY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Production revision"]
    #[inline(always)]
    pub fn prod_rev(&self) -> PROD_REV_R {
        PROD_REV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Part description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`part::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PART_SPEC;
impl crate::RegisterSpec for PART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`part::R`](R) reader structure"]
impl crate::Readable for PART_SPEC {}
