#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Flag Clear"]
pub type SINGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Flag Clear"]
pub type SCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Flag Clear"]
pub type SINGLEOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Flag Clear"]
pub type SCANOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<IFC_SPEC> {
        SINGLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<IFC_SPEC> {
        SCAN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IFC_SPEC> {
        SINGLEOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IFC_SPEC> {
        SCANOF_W::new(self, 9)
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
