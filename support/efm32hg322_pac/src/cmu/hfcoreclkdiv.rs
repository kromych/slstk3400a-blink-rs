#[doc = "Register `HFCORECLKDIV` reader"]
pub type R = crate::R<HfcoreclkdivSpec>;
#[doc = "Register `HFCORECLKDIV` writer"]
pub type W = crate::W<HfcoreclkdivSpec>;
#[doc = "HFCORECLK Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfcoreclkdiv {
    #[doc = "0: HFCORECLK = HFCLK."]
    Hfclk = 0,
    #[doc = "1: HFCORECLK = HFCLK/2."]
    Hfclk2 = 1,
    #[doc = "2: HFCORECLK = HFCLK/4."]
    Hfclk4 = 2,
    #[doc = "3: HFCORECLK = HFCLK/8."]
    Hfclk8 = 3,
    #[doc = "4: HFCORECLK = HFCLK/16."]
    Hfclk16 = 4,
    #[doc = "5: HFCORECLK = HFCLK/32."]
    Hfclk32 = 5,
    #[doc = "6: HFCORECLK = HFCLK/64."]
    Hfclk64 = 6,
    #[doc = "7: HFCORECLK = HFCLK/128."]
    Hfclk128 = 7,
    #[doc = "8: HFCORECLK = HFCLK/256."]
    Hfclk256 = 8,
    #[doc = "9: HFCORECLK = HFCLK/512."]
    Hfclk512 = 9,
}
impl From<Hfcoreclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Hfcoreclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfcoreclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Hfcoreclkdiv {}
#[doc = "Field `HFCORECLKDIV` reader - HFCORECLK Divider"]
pub type HfcoreclkdivR = crate::FieldReader<Hfcoreclkdiv>;
impl HfcoreclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfcoreclkdiv> {
        match self.bits {
            0 => Some(Hfcoreclkdiv::Hfclk),
            1 => Some(Hfcoreclkdiv::Hfclk2),
            2 => Some(Hfcoreclkdiv::Hfclk4),
            3 => Some(Hfcoreclkdiv::Hfclk8),
            4 => Some(Hfcoreclkdiv::Hfclk16),
            5 => Some(Hfcoreclkdiv::Hfclk32),
            6 => Some(Hfcoreclkdiv::Hfclk64),
            7 => Some(Hfcoreclkdiv::Hfclk128),
            8 => Some(Hfcoreclkdiv::Hfclk256),
            9 => Some(Hfcoreclkdiv::Hfclk512),
            _ => None,
        }
    }
    #[doc = "HFCORECLK = HFCLK."]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk
    }
    #[doc = "HFCORECLK = HFCLK/2."]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk2
    }
    #[doc = "HFCORECLK = HFCLK/4."]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk4
    }
    #[doc = "HFCORECLK = HFCLK/8."]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk8
    }
    #[doc = "HFCORECLK = HFCLK/16."]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk16
    }
    #[doc = "HFCORECLK = HFCLK/32."]
    #[inline(always)]
    pub fn is_hfclk32(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk32
    }
    #[doc = "HFCORECLK = HFCLK/64."]
    #[inline(always)]
    pub fn is_hfclk64(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk64
    }
    #[doc = "HFCORECLK = HFCLK/128."]
    #[inline(always)]
    pub fn is_hfclk128(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk128
    }
    #[doc = "HFCORECLK = HFCLK/256."]
    #[inline(always)]
    pub fn is_hfclk256(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk256
    }
    #[doc = "HFCORECLK = HFCLK/512."]
    #[inline(always)]
    pub fn is_hfclk512(&self) -> bool {
        *self == Hfcoreclkdiv::Hfclk512
    }
}
#[doc = "Field `HFCORECLKDIV` writer - HFCORECLK Divider"]
pub type HfcoreclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hfcoreclkdiv>;
impl<'a, REG> HfcoreclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFCORECLK = HFCLK."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk)
    }
    #[doc = "HFCORECLK = HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk2)
    }
    #[doc = "HFCORECLK = HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk4)
    }
    #[doc = "HFCORECLK = HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk8)
    }
    #[doc = "HFCORECLK = HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk16)
    }
    #[doc = "HFCORECLK = HFCLK/32."]
    #[inline(always)]
    pub fn hfclk32(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk32)
    }
    #[doc = "HFCORECLK = HFCLK/64."]
    #[inline(always)]
    pub fn hfclk64(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk64)
    }
    #[doc = "HFCORECLK = HFCLK/128."]
    #[inline(always)]
    pub fn hfclk128(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk128)
    }
    #[doc = "HFCORECLK = HFCLK/256."]
    #[inline(always)]
    pub fn hfclk256(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk256)
    }
    #[doc = "HFCORECLK = HFCLK/512."]
    #[inline(always)]
    pub fn hfclk512(self) -> &'a mut crate::W<REG> {
        self.variant(Hfcoreclkdiv::Hfclk512)
    }
}
#[doc = "Field `HFCORECLKLEDIV` reader - Additional Division Factor For HFCORECLKLE"]
pub type HfcoreclkledivR = crate::BitReader;
#[doc = "Field `HFCORECLKLEDIV` writer - Additional Division Factor For HFCORECLKLE"]
pub type HfcoreclkledivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline(always)]
    pub fn hfcoreclkdiv(&self) -> HfcoreclkdivR {
        HfcoreclkdivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline(always)]
    pub fn hfcoreclklediv(&self) -> HfcoreclkledivR {
        HfcoreclkledivR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HFCORECLK Divider"]
    #[inline(always)]
    pub fn hfcoreclkdiv(&mut self) -> HfcoreclkdivW<'_, HfcoreclkdivSpec> {
        HfcoreclkdivW::new(self, 0)
    }
    #[doc = "Bit 8 - Additional Division Factor For HFCORECLKLE"]
    #[inline(always)]
    pub fn hfcoreclklediv(&mut self) -> HfcoreclkledivW<'_, HfcoreclkdivSpec> {
        HfcoreclkledivW::new(self, 8)
    }
}
#[doc = "High Frequency Core Clock Division Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcoreclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcoreclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcoreclkdivSpec;
impl crate::RegisterSpec for HfcoreclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfcoreclkdiv::R`](R) reader structure"]
impl crate::Readable for HfcoreclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcoreclkdiv::W`](W) writer structure"]
impl crate::Writable for HfcoreclkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFCORECLKDIV to value 0"]
impl crate::Resettable for HfcoreclkdivSpec {}
