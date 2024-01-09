#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Current DAC Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Current DAC Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURSINK` reader - Current Sink Enable"]
pub type CURSINK_R = crate::BitReader;
#[doc = "Field `CURSINK` writer - Current Sink Enable"]
pub type CURSINK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINOUTTRANS` reader - Minimum Output Transition Enable"]
pub type MINOUTTRANS_R = crate::BitReader;
#[doc = "Field `MINOUTTRANS` writer - Minimum Output Transition Enable"]
pub type MINOUTTRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEN` reader - Output Enable"]
pub type OUTEN_R = crate::BitReader;
#[doc = "Field `OUTEN` writer - Output Enable"]
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTMODE` reader - Output Modes"]
pub type OUTMODE_R = crate::BitReader;
#[doc = "Field `OUTMODE` writer - Output Modes"]
pub type OUTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTENPRS` reader - PRS Controlled Output Enable"]
pub type OUTENPRS_R = crate::BitReader;
#[doc = "Field `OUTENPRS` writer - PRS Controlled Output Enable"]
pub type OUTENPRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSSEL` reader - IDAC Output PRS channnel Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "IDAC Output PRS channnel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL_A {
    type Ux = u8;
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
}
#[doc = "Field `PRSSEL` writer - IDAC Output PRS channnel Select"]
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PRSSEL_A>;
impl<'a, REG> PRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
}
impl R {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&self) -> CURSINK_R {
        CURSINK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&self) -> MINOUTTRANS_R {
        MINOUTTRANS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Modes"]
    #[inline(always)]
    pub fn outmode(&self) -> OUTMODE_R {
        OUTMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 18 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&self) -> OUTENPRS_R {
        OUTENPRS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - IDAC Output PRS channnel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cursink(&mut self) -> CURSINK_W<CTRL_SPEC> {
        CURSINK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minouttrans(&mut self) -> MINOUTTRANS_W<CTRL_SPEC> {
        MINOUTTRANS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<CTRL_SPEC> {
        OUTEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Output Modes"]
    #[inline(always)]
    #[must_use]
    pub fn outmode(&mut self) -> OUTMODE_W<CTRL_SPEC> {
        OUTMODE_W::new(self, 4)
    }
    #[doc = "Bit 18 - PRS Controlled Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outenprs(&mut self) -> OUTENPRS_W<CTRL_SPEC> {
        OUTENPRS_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - IDAC Output PRS channnel Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<CTRL_SPEC> {
        PRSSEL_W::new(self, 20)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
