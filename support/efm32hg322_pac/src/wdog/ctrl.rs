#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Watchdog Timer Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Watchdog Timer Enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM2RUN` reader - Energy Mode 2 Run Enable"]
pub type EM2RUN_R = crate::BitReader;
#[doc = "Field `EM2RUN` writer - Energy Mode 2 Run Enable"]
pub type EM2RUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM3RUN` reader - Energy Mode 3 Run Enable"]
pub type EM3RUN_R = crate::BitReader;
#[doc = "Field `EM3RUN` writer - Energy Mode 3 Run Enable"]
pub type EM3RUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK` reader - Configuration lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Configuration lock"]
pub type LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4BLOCK` reader - Energy Mode 4 Block"]
pub type EM4BLOCK_R = crate::BitReader;
#[doc = "Field `EM4BLOCK` writer - Energy Mode 4 Block"]
pub type EM4BLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWOSCBLOCK` reader - Software Oscillator Disable Block"]
pub type SWOSCBLOCK_R = crate::BitReader;
#[doc = "Field `SWOSCBLOCK` writer - Software Oscillator Disable Block"]
pub type SWOSCBLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERSEL` reader - Watchdog Timeout Period Select"]
pub type PERSEL_R = crate::FieldReader;
#[doc = "Field `PERSEL` writer - Watchdog Timeout Period Select"]
pub type PERSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLKSEL` reader - Watchdog Clock Select"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "Watchdog Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: ULFRCO"]
    ULFRCO = 0,
    #[doc = "1: LFRCO"]
    LFRCO = 1,
    #[doc = "2: LFXO"]
    LFXO = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::ULFRCO),
            1 => Some(CLKSEL_A::LFRCO),
            2 => Some(CLKSEL_A::LFXO),
            _ => None,
        }
    }
    #[doc = "ULFRCO"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKSEL_A::ULFRCO
    }
    #[doc = "LFRCO"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSEL_A::LFRCO
    }
    #[doc = "LFXO"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL_A::LFXO
    }
}
#[doc = "Field `CLKSEL` writer - Watchdog Clock Select"]
pub type CLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CLKSEL_A>;
impl<'a, REG, const O: u8> CLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ULFRCO"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::ULFRCO)
    }
    #[doc = "LFRCO"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::LFRCO)
    }
    #[doc = "LFXO"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::LFXO)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    pub fn em2run(&self) -> EM2RUN_R {
        EM2RUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    pub fn em3run(&self) -> EM3RUN_R {
        EM3RUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    pub fn em4block(&self) -> EM4BLOCK_R {
        EM4BLOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    pub fn swoscblock(&self) -> SWOSCBLOCK_R {
        SWOSCBLOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<CTRL_SPEC, 1> {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em2run(&mut self) -> EM2RUN_W<CTRL_SPEC, 2> {
        EM2RUN_W::new(self)
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em3run(&mut self) -> EM3RUN_W<CTRL_SPEC, 3> {
        EM3RUN_W::new(self)
    }
    #[doc = "Bit 4 - Configuration lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CTRL_SPEC, 4> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    #[must_use]
    pub fn em4block(&mut self) -> EM4BLOCK_W<CTRL_SPEC, 5> {
        EM4BLOCK_W::new(self)
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    #[must_use]
    pub fn swoscblock(&mut self) -> SWOSCBLOCK_W<CTRL_SPEC, 6> {
        SWOSCBLOCK_W::new(self)
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    #[must_use]
    pub fn persel(&mut self) -> PERSEL_W<CTRL_SPEC, 8> {
        PERSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CTRL_SPEC, 12> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0f00"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
