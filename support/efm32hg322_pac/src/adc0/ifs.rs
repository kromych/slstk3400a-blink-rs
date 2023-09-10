#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Flag Set"]
pub type SINGLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Flag Set"]
pub type SCAN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Flag Set"]
pub type SINGLEOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Flag Set"]
pub type SCANOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<IFS_SPEC, 0> {
        SINGLE_W::new(self)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<IFS_SPEC, 1> {
        SCAN_W::new(self)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IFS_SPEC, 8> {
        SINGLEOF_W::new(self)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IFS_SPEC, 9> {
        SCANOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
