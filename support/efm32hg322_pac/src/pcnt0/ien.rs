#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `UF` reader - Underflow Interrupt Enable"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - Underflow Interrupt Enable"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` reader - Overflow Interrupt Enable"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - Overflow Interrupt Enable"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` reader - Direction Change Detect Interrupt Enable"]
pub type DircngR = crate::BitReader;
#[doc = "Field `DIRCNG` writer - Direction Change Detect Interrupt Enable"]
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` reader - Auxiliary Overflow Interrupt Enable"]
pub type AuxofR = crate::BitReader;
#[doc = "Field `AUXOF` writer - Auxiliary Overflow Interrupt Enable"]
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` reader - Triggered compare Interrupt Enable"]
pub type TccR = crate::BitReader;
#[doc = "Field `TCC` writer - Triggered compare Interrupt Enable"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dircng(&self) -> DircngR {
        DircngR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn auxof(&self) -> AuxofR {
        AuxofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Enable"]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IenSpec> {
        UfW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IenSpec> {
        OfW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dircng(&mut self) -> DircngW<'_, IenSpec> {
        DircngW::new(self, 2)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn auxof(&mut self) -> AuxofW<'_, IenSpec> {
        AuxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Enable"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TccW<'_, IenSpec> {
        TccW::new(self, 4)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
