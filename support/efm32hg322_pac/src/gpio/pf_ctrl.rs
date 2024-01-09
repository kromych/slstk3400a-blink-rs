#[doc = "Register `PF_CTRL` reader"]
pub type R = crate::R<PF_CTRL_SPEC>;
#[doc = "Register `PF_CTRL` writer"]
pub type W = crate::W<PF_CTRL_SPEC>;
#[doc = "Field `DRIVEMODE` reader - Drive Mode Select"]
pub type DRIVEMODE_R = crate::FieldReader<DRIVEMODE_A>;
#[doc = "Drive Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRIVEMODE_A {
    #[doc = "0: 6 mA drive current"]
    STANDARD = 0,
    #[doc = "1: 0.1 mA drive current"]
    LOWEST = 1,
    #[doc = "2: 20 mA drive current"]
    HIGH = 2,
    #[doc = "3: 1 mA drive current"]
    LOW = 3,
}
impl From<DRIVEMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVEMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRIVEMODE_A {
    type Ux = u8;
}
impl DRIVEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRIVEMODE_A {
        match self.bits {
            0 => DRIVEMODE_A::STANDARD,
            1 => DRIVEMODE_A::LOWEST,
            2 => DRIVEMODE_A::HIGH,
            3 => DRIVEMODE_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "6 mA drive current"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DRIVEMODE_A::STANDARD
    }
    #[doc = "0.1 mA drive current"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == DRIVEMODE_A::LOWEST
    }
    #[doc = "20 mA drive current"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DRIVEMODE_A::HIGH
    }
    #[doc = "1 mA drive current"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DRIVEMODE_A::LOW
    }
}
#[doc = "Field `DRIVEMODE` writer - Drive Mode Select"]
pub type DRIVEMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DRIVEMODE_A>;
impl<'a, REG> DRIVEMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "6 mA drive current"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVEMODE_A::STANDARD)
    }
    #[doc = "0.1 mA drive current"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVEMODE_A::LOWEST)
    }
    #[doc = "20 mA drive current"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVEMODE_A::HIGH)
    }
    #[doc = "1 mA drive current"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVEMODE_A::LOW)
    }
}
impl R {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    pub fn drivemode(&self) -> DRIVEMODE_R {
        DRIVEMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Drive Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn drivemode(&mut self) -> DRIVEMODE_W<PF_CTRL_SPEC> {
        DRIVEMODE_W::new(self, 0)
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
#[doc = "Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_CTRL_SPEC;
impl crate::RegisterSpec for PF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_ctrl::R`](R) reader structure"]
impl crate::Readable for PF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pf_ctrl::W`](W) writer structure"]
impl crate::Writable for PF_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PF_CTRL to value 0"]
impl crate::Resettable for PF_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
