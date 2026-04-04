#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `UF` writer - Underflow interrupt set"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` writer - Overflow Interrupt Set"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` writer - Direction Change Detect Interrupt Set"]
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` writer - Auxiliary Overflow Interrupt Set"]
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Triggered compare Interrupt Set"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Underflow interrupt set"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IfsSpec> {
        UfW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Set"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfsSpec> {
        OfW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Set"]
    #[inline(always)]
    pub fn dircng(&mut self) -> DircngW<'_, IfsSpec> {
        DircngW::new(self, 2)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Set"]
    #[inline(always)]
    pub fn auxof(&mut self) -> AuxofW<'_, IfsSpec> {
        AuxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Set"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TccW<'_, IfsSpec> {
        TccW::new(self, 4)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
