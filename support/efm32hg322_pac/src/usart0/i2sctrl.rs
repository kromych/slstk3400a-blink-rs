#[doc = "Register `I2SCTRL` reader"]
pub type R = crate::R<I2SCTRL_SPEC>;
#[doc = "Register `I2SCTRL` writer"]
pub type W = crate::W<I2SCTRL_SPEC>;
#[doc = "Field `EN` reader - Enable I2S Mode"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable I2S Mode"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MONO` reader - Stero or Mono"]
pub type MONO_R = crate::BitReader;
#[doc = "Field `MONO` writer - Stero or Mono"]
pub type MONO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JUSTIFY` reader - Justification of I2S Data"]
pub type JUSTIFY_R = crate::BitReader;
#[doc = "Field `JUSTIFY` writer - Justification of I2S Data"]
pub type JUSTIFY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMASPLIT` reader - Separate DMA Request For Left/Right Data"]
pub type DMASPLIT_R = crate::BitReader;
#[doc = "Field `DMASPLIT` writer - Separate DMA Request For Left/Right Data"]
pub type DMASPLIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DELAY` reader - Delay on I2S data"]
pub type DELAY_R = crate::BitReader;
#[doc = "Field `DELAY` writer - Delay on I2S data"]
pub type DELAY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORMAT` reader - I2S Word Format"]
pub type FORMAT_R = crate::FieldReader<FORMAT_A>;
#[doc = "I2S Word Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: 32-bit word, 32-bit data"]
    W32D32 = 0,
    #[doc = "1: 32-bit word, 32-bit data with 8 lsb masked"]
    W32D24M = 1,
    #[doc = "2: 32-bit word, 24-bit data"]
    W32D24 = 2,
    #[doc = "3: 32-bit word, 16-bit data"]
    W32D16 = 3,
    #[doc = "4: 32-bit word, 8-bit data"]
    W32D8 = 4,
    #[doc = "5: 16-bit word, 16-bit data"]
    W16D16 = 5,
    #[doc = "6: 16-bit word, 8-bit data"]
    W16D8 = 6,
    #[doc = "7: 8-bit word, 8-bit data"]
    W8D8 = 7,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORMAT_A {
    type Ux = u8;
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            0 => FORMAT_A::W32D32,
            1 => FORMAT_A::W32D24M,
            2 => FORMAT_A::W32D24,
            3 => FORMAT_A::W32D16,
            4 => FORMAT_A::W32D8,
            5 => FORMAT_A::W16D16,
            6 => FORMAT_A::W16D8,
            7 => FORMAT_A::W8D8,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit word, 32-bit data"]
    #[inline(always)]
    pub fn is_w32d32(&self) -> bool {
        *self == FORMAT_A::W32D32
    }
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    #[inline(always)]
    pub fn is_w32d24m(&self) -> bool {
        *self == FORMAT_A::W32D24M
    }
    #[doc = "32-bit word, 24-bit data"]
    #[inline(always)]
    pub fn is_w32d24(&self) -> bool {
        *self == FORMAT_A::W32D24
    }
    #[doc = "32-bit word, 16-bit data"]
    #[inline(always)]
    pub fn is_w32d16(&self) -> bool {
        *self == FORMAT_A::W32D16
    }
    #[doc = "32-bit word, 8-bit data"]
    #[inline(always)]
    pub fn is_w32d8(&self) -> bool {
        *self == FORMAT_A::W32D8
    }
    #[doc = "16-bit word, 16-bit data"]
    #[inline(always)]
    pub fn is_w16d16(&self) -> bool {
        *self == FORMAT_A::W16D16
    }
    #[doc = "16-bit word, 8-bit data"]
    #[inline(always)]
    pub fn is_w16d8(&self) -> bool {
        *self == FORMAT_A::W16D8
    }
    #[doc = "8-bit word, 8-bit data"]
    #[inline(always)]
    pub fn is_w8d8(&self) -> bool {
        *self == FORMAT_A::W8D8
    }
}
#[doc = "Field `FORMAT` writer - I2S Word Format"]
pub type FORMAT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FORMAT_A>;
impl<'a, REG, const O: u8> FORMAT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit word, 32-bit data"]
    #[inline(always)]
    pub fn w32d32(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W32D32)
    }
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    #[inline(always)]
    pub fn w32d24m(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W32D24M)
    }
    #[doc = "32-bit word, 24-bit data"]
    #[inline(always)]
    pub fn w32d24(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W32D24)
    }
    #[doc = "32-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w32d16(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W32D16)
    }
    #[doc = "32-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w32d8(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W32D8)
    }
    #[doc = "16-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w16d16(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W16D16)
    }
    #[doc = "16-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w16d8(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W16D8)
    }
    #[doc = "8-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w8d8(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT_A::W8D8)
    }
}
impl R {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    pub fn justify(&self) -> JUSTIFY_R {
        JUSTIFY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline(always)]
    pub fn dmasplit(&self) -> DMASPLIT_R {
        DMASPLIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<I2SCTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<I2SCTRL_SPEC, 1> {
        MONO_W::new(self)
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    #[must_use]
    pub fn justify(&mut self) -> JUSTIFY_W<I2SCTRL_SPEC, 2> {
        JUSTIFY_W::new(self)
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline(always)]
    #[must_use]
    pub fn dmasplit(&mut self) -> DMASPLIT_W<I2SCTRL_SPEC, 3> {
        DMASPLIT_W::new(self)
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<I2SCTRL_SPEC, 4> {
        DELAY_W::new(self)
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<I2SCTRL_SPEC, 8> {
        FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCTRL_SPEC;
impl crate::RegisterSpec for I2SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sctrl::R`](R) reader structure"]
impl crate::Readable for I2SCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sctrl::W`](W) writer structure"]
impl crate::Writable for I2SCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCTRL to value 0"]
impl crate::Resettable for I2SCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
