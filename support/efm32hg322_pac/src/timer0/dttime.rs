#[doc = "Register `DTTIME` reader"]
pub type R = crate::R<DTTIME_SPEC>;
#[doc = "Register `DTTIME` writer"]
pub type W = crate::W<DTTIME_SPEC>;
#[doc = "Field `DTPRESC` reader - DTI Prescaler Setting"]
pub type DTPRESC_R = crate::FieldReader<DTPRESC_A>;
#[doc = "DTI Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRESC_A {
    #[doc = "0: The HFPERCLK is undivided"]
    DIV1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    DIV2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    DIV4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    DIV8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    DIV16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    DIV32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    DIV64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    DIV128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    DIV256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    DIV512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    DIV1024 = 10,
}
impl From<DTPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPRESC_A {
    type Ux = u8;
}
impl DTPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTPRESC_A> {
        match self.bits {
            0 => Some(DTPRESC_A::DIV1),
            1 => Some(DTPRESC_A::DIV2),
            2 => Some(DTPRESC_A::DIV4),
            3 => Some(DTPRESC_A::DIV8),
            4 => Some(DTPRESC_A::DIV16),
            5 => Some(DTPRESC_A::DIV32),
            6 => Some(DTPRESC_A::DIV64),
            7 => Some(DTPRESC_A::DIV128),
            8 => Some(DTPRESC_A::DIV256),
            9 => Some(DTPRESC_A::DIV512),
            10 => Some(DTPRESC_A::DIV1024),
            _ => None,
        }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DTPRESC_A::DIV1
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DTPRESC_A::DIV2
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DTPRESC_A::DIV4
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DTPRESC_A::DIV8
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DTPRESC_A::DIV16
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DTPRESC_A::DIV32
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DTPRESC_A::DIV64
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DTPRESC_A::DIV128
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == DTPRESC_A::DIV256
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == DTPRESC_A::DIV512
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == DTPRESC_A::DIV1024
    }
}
#[doc = "Field `DTPRESC` writer - DTI Prescaler Setting"]
pub type DTPRESC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DTPRESC_A>;
impl<'a, REG, const O: u8> DTPRESC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC_A::DIV1024)
    }
}
#[doc = "Field `DTRISET` reader - DTI Rise-time"]
pub type DTRISET_R = crate::FieldReader;
#[doc = "Field `DTRISET` writer - DTI Rise-time"]
pub type DTRISET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `DTFALLT` reader - DTI Fall-time"]
pub type DTFALLT_R = crate::FieldReader;
#[doc = "Field `DTFALLT` writer - DTI Fall-time"]
pub type DTFALLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&self) -> DTPRESC_R {
        DTPRESC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&self) -> DTRISET_R {
        DTRISET_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&self) -> DTFALLT_R {
        DTFALLT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dtpresc(&mut self) -> DTPRESC_W<DTTIME_SPEC, 0> {
        DTPRESC_W::new(self)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    #[must_use]
    pub fn dtriset(&mut self) -> DTRISET_W<DTTIME_SPEC, 8> {
        DTRISET_W::new(self)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    #[must_use]
    pub fn dtfallt(&mut self) -> DTFALLT_W<DTTIME_SPEC, 16> {
        DTFALLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DTI Time Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTTIME_SPEC;
impl crate::RegisterSpec for DTTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttime::R`](R) reader structure"]
impl crate::Readable for DTTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dttime::W`](W) writer structure"]
impl crate::Writable for DTTIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTTIME to value 0"]
impl crate::Resettable for DTTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
