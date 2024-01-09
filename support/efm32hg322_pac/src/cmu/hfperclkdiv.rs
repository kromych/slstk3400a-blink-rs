#[doc = "Register `HFPERCLKDIV` reader"]
pub type R = crate::R<HFPERCLKDIV_SPEC>;
#[doc = "Register `HFPERCLKDIV` writer"]
pub type W = crate::W<HFPERCLKDIV_SPEC>;
#[doc = "Field `HFPERCLKDIV` reader - HFPERCLK Divider"]
pub type HFPERCLKDIV_R = crate::FieldReader<HFPERCLKDIV_A>;
#[doc = "HFPERCLK Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFPERCLKDIV_A {
    #[doc = "0: HFPERCLK = HFCLK."]
    HFCLK = 0,
    #[doc = "1: HFPERCLK = HFCLK/2."]
    HFCLK2 = 1,
    #[doc = "2: HFPERCLK = HFCLK/4."]
    HFCLK4 = 2,
    #[doc = "3: HFPERCLK = HFCLK/8."]
    HFCLK8 = 3,
    #[doc = "4: HFPERCLK = HFCLK/16."]
    HFCLK16 = 4,
    #[doc = "5: HFPERCLK = HFCLK/32."]
    HFCLK32 = 5,
    #[doc = "6: HFPERCLK = HFCLK/64."]
    HFCLK64 = 6,
    #[doc = "7: HFPERCLK = HFCLK/128."]
    HFCLK128 = 7,
    #[doc = "8: HFPERCLK = HFCLK/256."]
    HFCLK256 = 8,
    #[doc = "9: HFPERCLK = HFCLK/512."]
    HFCLK512 = 9,
}
impl From<HFPERCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HFPERCLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HFPERCLKDIV_A {
    type Ux = u8;
}
impl HFPERCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HFPERCLKDIV_A> {
        match self.bits {
            0 => Some(HFPERCLKDIV_A::HFCLK),
            1 => Some(HFPERCLKDIV_A::HFCLK2),
            2 => Some(HFPERCLKDIV_A::HFCLK4),
            3 => Some(HFPERCLKDIV_A::HFCLK8),
            4 => Some(HFPERCLKDIV_A::HFCLK16),
            5 => Some(HFPERCLKDIV_A::HFCLK32),
            6 => Some(HFPERCLKDIV_A::HFCLK64),
            7 => Some(HFPERCLKDIV_A::HFCLK128),
            8 => Some(HFPERCLKDIV_A::HFCLK256),
            9 => Some(HFPERCLKDIV_A::HFCLK512),
            _ => None,
        }
    }
    #[doc = "HFPERCLK = HFCLK."]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK
    }
    #[doc = "HFPERCLK = HFCLK/2."]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK2
    }
    #[doc = "HFPERCLK = HFCLK/4."]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK4
    }
    #[doc = "HFPERCLK = HFCLK/8."]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK8
    }
    #[doc = "HFPERCLK = HFCLK/16."]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK16
    }
    #[doc = "HFPERCLK = HFCLK/32."]
    #[inline(always)]
    pub fn is_hfclk32(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK32
    }
    #[doc = "HFPERCLK = HFCLK/64."]
    #[inline(always)]
    pub fn is_hfclk64(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK64
    }
    #[doc = "HFPERCLK = HFCLK/128."]
    #[inline(always)]
    pub fn is_hfclk128(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK128
    }
    #[doc = "HFPERCLK = HFCLK/256."]
    #[inline(always)]
    pub fn is_hfclk256(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK256
    }
    #[doc = "HFPERCLK = HFCLK/512."]
    #[inline(always)]
    pub fn is_hfclk512(&self) -> bool {
        *self == HFPERCLKDIV_A::HFCLK512
    }
}
#[doc = "Field `HFPERCLKDIV` writer - HFPERCLK Divider"]
pub type HFPERCLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HFPERCLKDIV_A>;
impl<'a, REG> HFPERCLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFPERCLK = HFCLK."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK)
    }
    #[doc = "HFPERCLK = HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK2)
    }
    #[doc = "HFPERCLK = HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK4)
    }
    #[doc = "HFPERCLK = HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK8)
    }
    #[doc = "HFPERCLK = HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK16)
    }
    #[doc = "HFPERCLK = HFCLK/32."]
    #[inline(always)]
    pub fn hfclk32(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK32)
    }
    #[doc = "HFPERCLK = HFCLK/64."]
    #[inline(always)]
    pub fn hfclk64(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK64)
    }
    #[doc = "HFPERCLK = HFCLK/128."]
    #[inline(always)]
    pub fn hfclk128(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK128)
    }
    #[doc = "HFPERCLK = HFCLK/256."]
    #[inline(always)]
    pub fn hfclk256(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK256)
    }
    #[doc = "HFPERCLK = HFCLK/512."]
    #[inline(always)]
    pub fn hfclk512(self) -> &'a mut crate::W<REG> {
        self.variant(HFPERCLKDIV_A::HFCLK512)
    }
}
#[doc = "Field `HFPERCLKEN` reader - HFPERCLK Enable"]
pub type HFPERCLKEN_R = crate::BitReader;
#[doc = "Field `HFPERCLKEN` writer - HFPERCLK Enable"]
pub type HFPERCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    pub fn hfperclkdiv(&self) -> HFPERCLKDIV_R {
        HFPERCLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HFPERCLKEN_R {
        HFPERCLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    #[must_use]
    pub fn hfperclkdiv(&mut self) -> HFPERCLKDIV_W<HFPERCLKDIV_SPEC> {
        HFPERCLKDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfperclken(&mut self) -> HFPERCLKEN_W<HFPERCLKDIV_SPEC> {
        HFPERCLKEN_W::new(self, 8)
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
#[doc = "High Frequency Peripheral Clock Division Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPERCLKDIV_SPEC;
impl crate::RegisterSpec for HFPERCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclkdiv::R`](R) reader structure"]
impl crate::Readable for HFPERCLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfperclkdiv::W`](W) writer structure"]
impl crate::Writable for HFPERCLKDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFPERCLKDIV to value 0x0100"]
impl crate::Resettable for HFPERCLKDIV_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
