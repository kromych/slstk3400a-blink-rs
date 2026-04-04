#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `OF` writer - Overflow Interrupt Flag Clear"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` writer - Underflow Interrupt Flag Clear"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` writer - CC Channel 0 Interrupt Flag Clear"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` writer - CC Channel 1 Interrupt Flag Clear"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` writer - CC Channel 2 Interrupt Flag Clear"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF0` writer - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type Icbof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF1` writer - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type Icbof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF2` writer - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type Icbof2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfcSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IfcSpec> {
        UfW::new(self, 1)
    }
    #[doc = "Bit 4 - CC Channel 0 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn cc0(&mut self) -> Cc0W<'_, IfcSpec> {
        Cc0W::new(self, 4)
    }
    #[doc = "Bit 5 - CC Channel 1 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<'_, IfcSpec> {
        Cc1W::new(self, 5)
    }
    #[doc = "Bit 6 - CC Channel 2 Interrupt Flag Clear"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<'_, IfcSpec> {
        Cc2W::new(self, 6)
    }
    #[doc = "Bit 8 - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> Icbof0W<'_, IfcSpec> {
        Icbof0W::new(self, 8)
    }
    #[doc = "Bit 9 - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> Icbof1W<'_, IfcSpec> {
        Icbof1W::new(self, 9)
    }
    #[doc = "Bit 10 - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> Icbof2W<'_, IfcSpec> {
        Icbof2W::new(self, 10)
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
