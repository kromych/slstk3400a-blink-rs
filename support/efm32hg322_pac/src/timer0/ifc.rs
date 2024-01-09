#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `OF` writer - Overflow Interrupt Flag Clear"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` writer - Underflow Interrupt Flag Clear"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` writer - CC Channel 0 Interrupt Flag Clear"]
pub type CC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` writer - CC Channel 1 Interrupt Flag Clear"]
pub type CC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` writer - CC Channel 2 Interrupt Flag Clear"]
pub type CC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF0` writer - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type ICBOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF1` writer - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type ICBOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF2` writer - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag Clear"]
pub type ICBOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IFC_SPEC> {
        OF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IFC_SPEC> {
        UF_W::new(self, 1)
    }
    #[doc = "Bit 4 - CC Channel 0 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<IFC_SPEC> {
        CC0_W::new(self, 4)
    }
    #[doc = "Bit 5 - CC Channel 1 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<IFC_SPEC> {
        CC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - CC Channel 2 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<IFC_SPEC> {
        CC2_W::new(self, 6)
    }
    #[doc = "Bit 8 - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn icbof0(&mut self) -> ICBOF0_W<IFC_SPEC> {
        ICBOF0_W::new(self, 8)
    }
    #[doc = "Bit 9 - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn icbof1(&mut self) -> ICBOF1_W<IFC_SPEC> {
        ICBOF1_W::new(self, 9)
    }
    #[doc = "Bit 10 - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn icbof2(&mut self) -> ICBOF2_W<IFC_SPEC> {
        ICBOF2_W::new(self, 10)
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
