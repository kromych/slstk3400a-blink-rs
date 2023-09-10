#[doc = "Register `FRAME` reader"]
pub type R = crate::R<FRAME_SPEC>;
#[doc = "Register `FRAME` writer"]
pub type W = crate::W<FRAME_SPEC>;
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DATABITS_R = crate::FieldReader<DATABITS_A>;
#[doc = "Data-Bit Mode\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATABITS_A {
    #[doc = "1: Each frame contains 4 data bits"]
    FOUR = 1,
    #[doc = "2: Each frame contains 5 data bits"]
    FIVE = 2,
    #[doc = "3: Each frame contains 6 data bits"]
    SIX = 3,
    #[doc = "4: Each frame contains 7 data bits"]
    SEVEN = 4,
    #[doc = "5: Each frame contains 8 data bits"]
    EIGHT = 5,
    #[doc = "6: Each frame contains 9 data bits"]
    NINE = 6,
    #[doc = "7: Each frame contains 10 data bits"]
    TEN = 7,
    #[doc = "8: Each frame contains 11 data bits"]
    ELEVEN = 8,
    #[doc = "9: Each frame contains 12 data bits"]
    TWELVE = 9,
    #[doc = "10: Each frame contains 13 data bits"]
    THIRTEEN = 10,
    #[doc = "11: Each frame contains 14 data bits"]
    FOURTEEN = 11,
    #[doc = "12: Each frame contains 15 data bits"]
    FIFTEEN = 12,
    #[doc = "13: Each frame contains 16 data bits"]
    SIXTEEN = 13,
}
impl From<DATABITS_A> for u8 {
    #[inline(always)]
    fn from(variant: DATABITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATABITS_A {
    type Ux = u8;
}
impl DATABITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATABITS_A> {
        match self.bits {
            1 => Some(DATABITS_A::FOUR),
            2 => Some(DATABITS_A::FIVE),
            3 => Some(DATABITS_A::SIX),
            4 => Some(DATABITS_A::SEVEN),
            5 => Some(DATABITS_A::EIGHT),
            6 => Some(DATABITS_A::NINE),
            7 => Some(DATABITS_A::TEN),
            8 => Some(DATABITS_A::ELEVEN),
            9 => Some(DATABITS_A::TWELVE),
            10 => Some(DATABITS_A::THIRTEEN),
            11 => Some(DATABITS_A::FOURTEEN),
            12 => Some(DATABITS_A::FIFTEEN),
            13 => Some(DATABITS_A::SIXTEEN),
            _ => None,
        }
    }
    #[doc = "Each frame contains 4 data bits"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DATABITS_A::FOUR
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == DATABITS_A::FIVE
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == DATABITS_A::SIX
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == DATABITS_A::SEVEN
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DATABITS_A::EIGHT
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn is_nine(&self) -> bool {
        *self == DATABITS_A::NINE
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline(always)]
    pub fn is_ten(&self) -> bool {
        *self == DATABITS_A::TEN
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline(always)]
    pub fn is_eleven(&self) -> bool {
        *self == DATABITS_A::ELEVEN
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline(always)]
    pub fn is_twelve(&self) -> bool {
        *self == DATABITS_A::TWELVE
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline(always)]
    pub fn is_thirteen(&self) -> bool {
        *self == DATABITS_A::THIRTEEN
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline(always)]
    pub fn is_fourteen(&self) -> bool {
        *self == DATABITS_A::FOURTEEN
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline(always)]
    pub fn is_fifteen(&self) -> bool {
        *self == DATABITS_A::FIFTEEN
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == DATABITS_A::SIXTEEN
    }
}
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DATABITS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DATABITS_A>;
impl<'a, REG, const O: u8> DATABITS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each frame contains 4 data bits"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::FOUR)
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::FIVE)
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::SIX)
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::SEVEN)
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::EIGHT)
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn nine(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::NINE)
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline(always)]
    pub fn ten(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::TEN)
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline(always)]
    pub fn eleven(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::ELEVEN)
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline(always)]
    pub fn twelve(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::TWELVE)
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline(always)]
    pub fn thirteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::THIRTEEN)
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline(always)]
    pub fn fourteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::FOURTEEN)
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline(always)]
    pub fn fifteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::FIFTEEN)
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(DATABITS_A::SIXTEEN)
    }
}
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type PARITY_R = crate::FieldReader<PARITY_A>;
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Parity bits are not used"]
    NONE = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD = 3,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PARITY_A {
    type Ux = u8;
}
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITY_A> {
        match self.bits {
            0 => Some(PARITY_A::NONE),
            2 => Some(PARITY_A::EVEN),
            3 => Some(PARITY_A::ODD),
            _ => None,
        }
    }
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY_A::NONE
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_A::EVEN
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_A::ODD
    }
}
#[doc = "Field `PARITY` writer - Parity-Bit Mode"]
pub type PARITY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PARITY_A>;
impl<'a, REG, const O: u8> PARITY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_A::NONE)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_A::EVEN)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_A::ODD)
    }
}
#[doc = "Field `STOPBITS` reader - Stop-Bit Mode"]
pub type STOPBITS_R = crate::FieldReader<STOPBITS_A>;
#[doc = "Stop-Bit Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOPBITS_A {
    #[doc = "0: The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    HALF = 0,
    #[doc = "1: One stop bit is generated and verified"]
    ONE = 1,
    #[doc = "2: The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    ONEANDAHALF = 2,
    #[doc = "3: The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    TWO = 3,
}
impl From<STOPBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPBITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOPBITS_A {
    type Ux = u8;
}
impl STOPBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPBITS_A {
        match self.bits {
            0 => STOPBITS_A::HALF,
            1 => STOPBITS_A::ONE,
            2 => STOPBITS_A::ONEANDAHALF,
            3 => STOPBITS_A::TWO,
            _ => unreachable!(),
        }
    }
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == STOPBITS_A::HALF
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOPBITS_A::ONE
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn is_oneandahalf(&self) -> bool {
        *self == STOPBITS_A::ONEANDAHALF
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOPBITS_A::TWO
    }
}
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type STOPBITS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, STOPBITS_A>;
impl<'a, REG, const O: u8> STOPBITS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS_A::HALF)
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS_A::ONE)
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn oneandahalf(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS_A::ONEANDAHALF)
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBITS_A::TWO)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DATABITS_R {
        DATABITS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn databits(&mut self) -> DATABITS_W<FRAME_SPEC, 0> {
        DATABITS_W::new(self)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<FRAME_SPEC, 8> {
        PARITY_W::new(self)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stopbits(&mut self) -> STOPBITS_W<FRAME_SPEC, 12> {
        STOPBITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USART Frame Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAME_SPEC;
impl crate::RegisterSpec for FRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame::R`](R) reader structure"]
impl crate::Readable for FRAME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frame::W`](W) writer structure"]
impl crate::Writable for FRAME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAME to value 0x1005"]
impl crate::Resettable for FRAME_SPEC {
    const RESET_VALUE: Self::Ux = 0x1005;
}
