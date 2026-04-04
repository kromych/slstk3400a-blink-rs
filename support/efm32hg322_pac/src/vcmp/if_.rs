#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `EDGE` reader - Edge Triggered Interrupt Flag"]
pub type EdgeR = crate::BitReader;
#[doc = "Field `WARMUP` reader - Warm-up Interrupt Flag"]
pub type WarmupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Flag"]
    #[inline(always)]
    pub fn warmup(&self) -> WarmupR {
        WarmupR::new(((self.bits >> 1) & 1) != 0)
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
