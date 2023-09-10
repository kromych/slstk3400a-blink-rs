#[doc = "Register `MSIZE` reader"]
pub type R = crate::R<MSIZE_SPEC>;
#[doc = "Field `FLASH` reader - Flash size in kilobytes"]
pub type FLASH_R = crate::FieldReader<u16>;
#[doc = "Field `SRAM` reader - SRAM size in kilobytes"]
pub type SRAM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Flash size in kilobytes"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM size in kilobytes"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Flash and SRAM Memory size in KiloBytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msize::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSIZE_SPEC;
impl crate::RegisterSpec for MSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msize::R`](R) reader structure"]
impl crate::Readable for MSIZE_SPEC {}
