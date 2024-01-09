#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `UF` reader - Underflow Interrupt Enable"]
pub type UF_R = crate::BitReader;
#[doc = "Field `UF` writer - Underflow Interrupt Enable"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` reader - Overflow Interrupt Enable"]
pub type OF_R = crate::BitReader;
#[doc = "Field `OF` writer - Overflow Interrupt Enable"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` reader - Direction Change Detect Interrupt Enable"]
pub type DIRCNG_R = crate::BitReader;
#[doc = "Field `DIRCNG` writer - Direction Change Detect Interrupt Enable"]
pub type DIRCNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` reader - Auxiliary Overflow Interrupt Enable"]
pub type AUXOF_R = crate::BitReader;
#[doc = "Field `AUXOF` writer - Auxiliary Overflow Interrupt Enable"]
pub type AUXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` reader - Triggered compare Interrupt Enable"]
pub type TCC_R = crate::BitReader;
#[doc = "Field `TCC` writer - Triggered compare Interrupt Enable"]
pub type TCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dircng(&self) -> DIRCNG_R {
        DIRCNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn auxof(&self) -> AUXOF_R {
        AUXOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Enable"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IEN_SPEC> {
        UF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IEN_SPEC> {
        OF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DIRCNG_W<IEN_SPEC> {
        DIRCNG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AUXOF_W<IEN_SPEC> {
        AUXOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Triggered compare Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<IEN_SPEC> {
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
