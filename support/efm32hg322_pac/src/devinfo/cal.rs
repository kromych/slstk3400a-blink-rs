#[doc = "Register `CAL` reader"]
pub type R = crate::R<CAL_SPEC>;
#[doc = "Field `CRC` reader - Integrity CRC checksum"]
pub type CRC_R = crate::FieldReader<u16>;
#[doc = "Field `TEMP` reader - Calibration temperature, DegC"]
pub type TEMP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Integrity CRC checksum"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Calibration temperature, DegC"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Calibration temperature and checksum\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CAL_SPEC {}
