#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `VREGOSH` reader - VREGO Sense High Interrupt Flag"]
pub type VREGOSH_R = crate::BitReader;
#[doc = "Field `VREGOSL` reader - VREGO Sense Low Interrupt Flag"]
pub type VREGOSL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Flag"]
    #[inline(always)]
    pub fn vregosh(&self) -> VREGOSH_R {
        VREGOSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Flag"]
    #[inline(always)]
    pub fn vregosl(&self) -> VREGOSL_R {
        VREGOSL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0x03"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
