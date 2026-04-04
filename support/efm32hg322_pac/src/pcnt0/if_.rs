#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `UF` reader - Underflow Interrupt Read Flag"]
pub type UfR = crate::BitReader;
#[doc = "Field `OF` reader - Overflow Interrupt Read Flag"]
pub type OfR = crate::BitReader;
#[doc = "Field `DIRCNG` reader - Direction Change Detect Interrupt Flag"]
pub type DircngR = crate::BitReader;
#[doc = "Field `AUXOF` reader - Overflow Interrupt Read Flag"]
pub type AuxofR = crate::BitReader;
#[doc = "Field `TCC` reader - Triggered compare Interrupt Read Flag"]
pub type TccR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Underflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dircng(&self) -> DircngR {
        DircngR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn auxof(&self) -> AuxofR {
        AuxofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Read Flag"]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 4) & 1) != 0)
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
