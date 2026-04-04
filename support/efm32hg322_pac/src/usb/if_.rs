#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `VREGOSH` reader - VREGO Sense High Interrupt Flag"]
pub type VregoshR = crate::BitReader;
#[doc = "Field `VREGOSL` reader - VREGO Sense Low Interrupt Flag"]
pub type VregoslR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Flag"]
    #[inline(always)]
    pub fn vregosh(&self) -> VregoshR {
        VregoshR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Flag"]
    #[inline(always)]
    pub fn vregosl(&self) -> VregoslR {
        VregoslR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0x03"]
impl crate::Resettable for IfSpec {
    const RESET_VALUE: u32 = 0x03;
}
