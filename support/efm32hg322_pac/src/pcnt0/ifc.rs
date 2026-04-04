#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `UF` writer - Underflow Interrupt Clear"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` writer - Overflow Interrupt Clear"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` writer - Direction Change Detect Interrupt Clear"]
pub type DircngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` writer - Auxiliary Overflow Interrupt Clear"]
pub type AuxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Triggered compare Interrupt Clear"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IfcSpec> {
        UfW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfcSpec> {
        OfW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Clear"]
    #[inline(always)]
    pub fn dircng(&mut self) -> DircngW<'_, IfcSpec> {
        DircngW::new(self, 2)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn auxof(&mut self) -> AuxofW<'_, IfcSpec> {
        AuxofW::new(self, 3)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Clear"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TccW<'_, IfcSpec> {
        TccW::new(self, 4)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
