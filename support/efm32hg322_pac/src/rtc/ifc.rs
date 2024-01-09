#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `OF` writer - Clear Overflow Interrupt Flag"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0` writer - Clear Compare match 0 Interrupt Flag"]
pub type COMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` writer - Clear Compare match 1 Interrupt Flag"]
pub type COMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IFC_SPEC> {
        OF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Compare match 0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<IFC_SPEC> {
        COMP0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Compare match 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<IFC_SPEC> {
        COMP1_W::new(self, 2)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
