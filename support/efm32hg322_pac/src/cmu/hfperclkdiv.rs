#[doc = "Register `HFPERCLKDIV` reader"]
pub type R = crate::R<HfperclkdivSpec>;
#[doc = "Register `HFPERCLKDIV` writer"]
pub type W = crate::W<HfperclkdivSpec>;
#[doc = "HFPERCLK Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfperclkdiv {
    #[doc = "0: HFPERCLK = HFCLK."]
    Hfclk = 0,
    #[doc = "1: HFPERCLK = HFCLK/2."]
    Hfclk2 = 1,
    #[doc = "2: HFPERCLK = HFCLK/4."]
    Hfclk4 = 2,
    #[doc = "3: HFPERCLK = HFCLK/8."]
    Hfclk8 = 3,
    #[doc = "4: HFPERCLK = HFCLK/16."]
    Hfclk16 = 4,
    #[doc = "5: HFPERCLK = HFCLK/32."]
    Hfclk32 = 5,
    #[doc = "6: HFPERCLK = HFCLK/64."]
    Hfclk64 = 6,
    #[doc = "7: HFPERCLK = HFCLK/128."]
    Hfclk128 = 7,
    #[doc = "8: HFPERCLK = HFCLK/256."]
    Hfclk256 = 8,
    #[doc = "9: HFPERCLK = HFCLK/512."]
    Hfclk512 = 9,
}
impl From<Hfperclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Hfperclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfperclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Hfperclkdiv {}
#[doc = "Field `HFPERCLKDIV` reader - HFPERCLK Divider"]
pub type HfperclkdivR = crate::FieldReader<Hfperclkdiv>;
impl HfperclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfperclkdiv> {
        match self.bits {
            0 => Some(Hfperclkdiv::Hfclk),
            1 => Some(Hfperclkdiv::Hfclk2),
            2 => Some(Hfperclkdiv::Hfclk4),
            3 => Some(Hfperclkdiv::Hfclk8),
            4 => Some(Hfperclkdiv::Hfclk16),
            5 => Some(Hfperclkdiv::Hfclk32),
            6 => Some(Hfperclkdiv::Hfclk64),
            7 => Some(Hfperclkdiv::Hfclk128),
            8 => Some(Hfperclkdiv::Hfclk256),
            9 => Some(Hfperclkdiv::Hfclk512),
            _ => None,
        }
    }
    #[doc = "HFPERCLK = HFCLK."]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Hfperclkdiv::Hfclk
    }
    #[doc = "HFPERCLK = HFCLK/2."]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == Hfperclkdiv::Hfclk2
    }
    #[doc = "HFPERCLK = HFCLK/4."]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == Hfperclkdiv::Hfclk4
    }
    #[doc = "HFPERCLK = HFCLK/8."]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == Hfperclkdiv::Hfclk8
    }
    #[doc = "HFPERCLK = HFCLK/16."]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == Hfperclkdiv::Hfclk16
    }
    #[doc = "HFPERCLK = HFCLK/32."]
    #[inline(always)]
    pub fn is_hfclk32(&self) -> bool {
        *self == Hfperclkdiv::Hfclk32
    }
    #[doc = "HFPERCLK = HFCLK/64."]
    #[inline(always)]
    pub fn is_hfclk64(&self) -> bool {
        *self == Hfperclkdiv::Hfclk64
    }
    #[doc = "HFPERCLK = HFCLK/128."]
    #[inline(always)]
    pub fn is_hfclk128(&self) -> bool {
        *self == Hfperclkdiv::Hfclk128
    }
    #[doc = "HFPERCLK = HFCLK/256."]
    #[inline(always)]
    pub fn is_hfclk256(&self) -> bool {
        *self == Hfperclkdiv::Hfclk256
    }
    #[doc = "HFPERCLK = HFCLK/512."]
    #[inline(always)]
    pub fn is_hfclk512(&self) -> bool {
        *self == Hfperclkdiv::Hfclk512
    }
}
#[doc = "Field `HFPERCLKDIV` writer - HFPERCLK Divider"]
pub type HfperclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hfperclkdiv>;
impl<'a, REG> HfperclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFPERCLK = HFCLK."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk)
    }
    #[doc = "HFPERCLK = HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk2)
    }
    #[doc = "HFPERCLK = HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk4)
    }
    #[doc = "HFPERCLK = HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk8)
    }
    #[doc = "HFPERCLK = HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk16)
    }
    #[doc = "HFPERCLK = HFCLK/32."]
    #[inline(always)]
    pub fn hfclk32(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk32)
    }
    #[doc = "HFPERCLK = HFCLK/64."]
    #[inline(always)]
    pub fn hfclk64(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk64)
    }
    #[doc = "HFPERCLK = HFCLK/128."]
    #[inline(always)]
    pub fn hfclk128(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk128)
    }
    #[doc = "HFPERCLK = HFCLK/256."]
    #[inline(always)]
    pub fn hfclk256(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk256)
    }
    #[doc = "HFPERCLK = HFCLK/512."]
    #[inline(always)]
    pub fn hfclk512(self) -> &'a mut crate::W<REG> {
        self.variant(Hfperclkdiv::Hfclk512)
    }
}
#[doc = "Field `HFPERCLKEN` reader - HFPERCLK Enable"]
pub type HfperclkenR = crate::BitReader;
#[doc = "Field `HFPERCLKEN` writer - HFPERCLK Enable"]
pub type HfperclkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    pub fn hfperclkdiv(&self) -> HfperclkdivR {
        HfperclkdivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HfperclkenR {
        HfperclkenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HFPERCLK Divider"]
    #[inline(always)]
    pub fn hfperclkdiv(&mut self) -> HfperclkdivW<'_, HfperclkdivSpec> {
        HfperclkdivW::new(self, 0)
    }
    #[doc = "Bit 8 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&mut self) -> HfperclkenW<'_, HfperclkdivSpec> {
        HfperclkenW::new(self, 8)
    }
}
#[doc = "High Frequency Peripheral Clock Division Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfperclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfperclkdivSpec;
impl crate::RegisterSpec for HfperclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclkdiv::R`](R) reader structure"]
impl crate::Readable for HfperclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`hfperclkdiv::W`](W) writer structure"]
impl crate::Writable for HfperclkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFPERCLKDIV to value 0x0100"]
impl crate::Resettable for HfperclkdivSpec {
    const RESET_VALUE: u32 = 0x0100;
}
