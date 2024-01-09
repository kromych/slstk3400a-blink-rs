#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `UF` writer - Underflow Interrupt Clear"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` writer - Overflow Interrupt Clear"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` writer - Direction Change Detect Interrupt Clear"]
pub type DIRCNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` writer - Auxiliary Overflow Interrupt Clear"]
pub type AUXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Triggered compare Interrupt Clear"]
pub type TCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IFC_SPEC> {
        UF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IFC_SPEC> {
        OF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DIRCNG_W<IFC_SPEC> {
        DIRCNG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AUXOF_W<IFC_SPEC> {
        AUXOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<IFC_SPEC> {
        TCC_W::new(self, 4)
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
