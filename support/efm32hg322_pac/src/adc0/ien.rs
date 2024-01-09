#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Enable"]
pub type SINGLE_R = crate::BitReader;
#[doc = "Field `SINGLE` writer - Single Conversion Complete Interrupt Enable"]
pub type SINGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Enable"]
pub type SCAN_R = crate::BitReader;
#[doc = "Field `SCAN` writer - Scan Conversion Complete Interrupt Enable"]
pub type SCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEOF` reader - Single Result Overflow Interrupt Enable"]
pub type SINGLEOF_R = crate::BitReader;
#[doc = "Field `SINGLEOF` writer - Single Result Overflow Interrupt Enable"]
pub type SINGLEOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` reader - Scan Result Overflow Interrupt Enable"]
pub type SCANOF_R = crate::BitReader;
#[doc = "Field `SCANOF` writer - Scan Result Overflow Interrupt Enable"]
pub type SCANOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<IEN_SPEC> {
        SINGLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<IEN_SPEC> {
        SCAN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IEN_SPEC> {
        SINGLEOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IEN_SPEC> {
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
