#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Voltage Supply Comparator Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Voltage Supply Comparator Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTVAL` reader - Inactive Value"]
pub type INACTVAL_R = crate::BitReader;
#[doc = "Field `INACTVAL` writer - Inactive Value"]
pub type INACTVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTEN` reader - Hysteresis Enable"]
pub type HYSTEN_R = crate::BitReader;
#[doc = "Field `HYSTEN` writer - Hysteresis Enable"]
pub type HYSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMTIME` reader - Warm-Up Time"]
pub type WARMTIME_R = crate::FieldReader<WARMTIME_A>;
#[doc = "Warm-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMTIME_A {
    #[doc = "0: 4 HFPERCLK cycles"]
    _4CYCLES = 0,
    #[doc = "1: 8 HFPERCLK cycles"]
    _8CYCLES = 1,
    #[doc = "2: 16 HFPERCLK cycles"]
    _16CYCLES = 2,
    #[doc = "3: 32 HFPERCLK cycles"]
    _32CYCLES = 3,
    #[doc = "4: 64 HFPERCLK cycles"]
    _64CYCLES = 4,
    #[doc = "5: 128 HFPERCLK cycles"]
    _128CYCLES = 5,
    #[doc = "6: 256 HFPERCLK cycles"]
    _256CYCLES = 6,
    #[doc = "7: 512 HFPERCLK cycles"]
    _512CYCLES = 7,
}
impl From<WARMTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMTIME_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WARMTIME_A {
    type Ux = u8;
}
impl WARMTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WARMTIME_A {
        match self.bits {
            0 => WARMTIME_A::_4CYCLES,
            1 => WARMTIME_A::_8CYCLES,
            2 => WARMTIME_A::_16CYCLES,
            3 => WARMTIME_A::_32CYCLES,
            4 => WARMTIME_A::_64CYCLES,
            5 => WARMTIME_A::_128CYCLES,
            6 => WARMTIME_A::_256CYCLES,
            7 => WARMTIME_A::_512CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "4 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == WARMTIME_A::_4CYCLES
    }
    #[doc = "8 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == WARMTIME_A::_8CYCLES
    }
    #[doc = "16 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == WARMTIME_A::_16CYCLES
    }
    #[doc = "32 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == WARMTIME_A::_32CYCLES
    }
    #[doc = "64 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == WARMTIME_A::_64CYCLES
    }
    #[doc = "128 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == WARMTIME_A::_128CYCLES
    }
    #[doc = "256 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == WARMTIME_A::_256CYCLES
    }
    #[doc = "512 HFPERCLK cycles"]
    #[inline(always)]
    pub fn is_512cycles(&self) -> bool {
        *self == WARMTIME_A::_512CYCLES
    }
}
#[doc = "Field `WARMTIME` writer - Warm-Up Time"]
pub type WARMTIME_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, WARMTIME_A>;
impl<'a, REG> WARMTIME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_4CYCLES)
    }
    #[doc = "8 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_8CYCLES)
    }
    #[doc = "16 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_16CYCLES)
    }
    #[doc = "32 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_32CYCLES)
    }
    #[doc = "64 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_64CYCLES)
    }
    #[doc = "128 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_128CYCLES)
    }
    #[doc = "256 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_256CYCLES)
    }
    #[doc = "512 HFPERCLK cycles"]
    #[inline(always)]
    pub fn _512cycles(self) -> &'a mut crate::W<REG> {
        self.variant(WARMTIME_A::_512CYCLES)
    }
}
#[doc = "Field `IRISE` reader - Rising Edge Interrupt Sense"]
pub type IRISE_R = crate::BitReader;
#[doc = "Field `IRISE` writer - Rising Edge Interrupt Sense"]
pub type IRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFALL` reader - Falling Edge Interrupt Sense"]
pub type IFALL_R = crate::BitReader;
#[doc = "Field `IFALL` writer - Falling Edge Interrupt Sense"]
pub type IFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIASPROG` reader - VCMP Bias Programming Value"]
pub type BIASPROG_R = crate::FieldReader;
#[doc = "Field `BIASPROG` writer - VCMP Bias Programming Value"]
pub type BIASPROG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub type HALFBIAS_R = crate::BitReader;
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub type HALFBIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> INACTVAL_R {
        INACTVAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hysten(&self) -> HYSTEN_R {
        HYSTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
    #[inline(always)]
    pub fn warmtime(&self) -> WARMTIME_R {
        WARMTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&self) -> IRISE_R {
        IRISE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&self) -> IFALL_R {
        IFALL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Supply Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    #[must_use]
    pub fn inactval(&mut self) -> INACTVAL_W<CTRL_SPEC> {
        INACTVAL_W::new(self, 2)
    }
    #[doc = "Bit 4 - Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hysten(&mut self) -> HYSTEN_W<CTRL_SPEC> {
        HYSTEN_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Warm-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn warmtime(&mut self) -> WARMTIME_W<CTRL_SPEC> {
        WARMTIME_W::new(self, 8)
    }
    #[doc = "Bit 16 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    #[must_use]
    pub fn irise(&mut self) -> IRISE_W<CTRL_SPEC> {
        IRISE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    #[must_use]
    pub fn ifall(&mut self) -> IFALL_W<CTRL_SPEC> {
        IFALL_W::new(self, 17)
    }
    #[doc = "Bits 24:27 - VCMP Bias Programming Value"]
    #[inline(always)]
    #[must_use]
    pub fn biasprog(&mut self) -> BIASPROG_W<CTRL_SPEC> {
        BIASPROG_W::new(self, 24)
    }
    #[doc = "Bit 30 - Half Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn halfbias(&mut self) -> HALFBIAS_W<CTRL_SPEC> {
        HALFBIAS_W::new(self, 30)
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
#[doc = "`reset()` method sets CTRL to value 0x4700_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4700_0000;
}
