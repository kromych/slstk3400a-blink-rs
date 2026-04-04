#[doc = "Register `MSIZE` reader"]
pub type R = crate::R<MsizeSpec>;
#[doc = "Field `FLASH` reader - Flash size in kilobytes"]
pub type FlashR = crate::FieldReader<u16>;
#[doc = "Field `SRAM` reader - SRAM size in kilobytes"]
pub type SramR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Flash size in kilobytes"]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM size in kilobytes"]
    #[inline(always)]
    pub fn sram(&self) -> SramR {
        SramR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Flash and SRAM Memory size in KiloBytes\n\nYou can [`read`](crate::Reg::read) this register and get [`msize::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsizeSpec;
impl crate::RegisterSpec for MsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msize::R`](R) reader structure"]
impl crate::Readable for MsizeSpec {}
