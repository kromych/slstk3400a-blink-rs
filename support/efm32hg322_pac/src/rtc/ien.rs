#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `OF` reader - Overflow Interrupt Enable"]
pub type OF_R = crate::BitReader;
#[doc = "Field `OF` writer - Overflow Interrupt Enable"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP0` reader - Compare Match 0 Interrupt Enable"]
pub type COMP0_R = crate::BitReader;
#[doc = "Field `COMP0` writer - Compare Match 0 Interrupt Enable"]
pub type COMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` reader - Compare Match 1 Interrupt Enable"]
pub type COMP1_R = crate::BitReader;
#[doc = "Field `COMP1` writer - Compare Match 1 Interrupt Enable"]
pub type COMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IEN_SPEC> {
        OF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare Match 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<IEN_SPEC> {
        COMP0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare Match 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<IEN_SPEC> {
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
