#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `OF` writer - Set Overflow Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0` writer - Set Compare match 0 Interrupt Flag"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` writer - Set Compare match 1 Interrupt Flag"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfsSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - Set Compare match 0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&mut self) -> Comp0W<'_, IfsSpec> {
        Comp0W::new(self, 1)
    }
    #[doc = "Bit 2 - Set Compare match 1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&mut self) -> Comp1W<'_, IfsSpec> {
        Comp1W::new(self, 2)
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
