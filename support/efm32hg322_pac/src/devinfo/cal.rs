#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Field `CRC` reader - Integrity CRC checksum"]
pub type CrcR = crate::FieldReader<u16>;
#[doc = "Field `TEMP` reader - Calibration temperature, DegC"]
pub type TempR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Integrity CRC checksum"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Calibration temperature, DegC"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Calibration temperature and checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
