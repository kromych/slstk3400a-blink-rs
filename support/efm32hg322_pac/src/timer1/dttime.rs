#[doc = "Register `DTTIME` reader"]
pub type R = crate::R<DttimeSpec>;
#[doc = "Register `DTTIME` writer"]
pub type W = crate::W<DttimeSpec>;
#[doc = "DTI Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtpresc {
    #[doc = "0: The HFPERCLK is undivided"]
    Div1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    Div2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    Div4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    Div8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    Div16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    Div32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    Div64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    Div128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    Div256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    Div512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    Div1024 = 10,
}
impl From<Dtpresc> for u8 {
    #[inline(always)]
    fn from(variant: Dtpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtpresc {
    type Ux = u8;
}
impl crate::IsEnum for Dtpresc {}
#[doc = "Field `DTPRESC` reader - DTI Prescaler Setting"]
pub type DtprescR = crate::FieldReader<Dtpresc>;
impl DtprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtpresc> {
        match self.bits {
            0 => Some(Dtpresc::Div1),
            1 => Some(Dtpresc::Div2),
            2 => Some(Dtpresc::Div4),
            3 => Some(Dtpresc::Div8),
            4 => Some(Dtpresc::Div16),
            5 => Some(Dtpresc::Div32),
            6 => Some(Dtpresc::Div64),
            7 => Some(Dtpresc::Div128),
            8 => Some(Dtpresc::Div256),
            9 => Some(Dtpresc::Div512),
            10 => Some(Dtpresc::Div1024),
            _ => None,
        }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Dtpresc::Div1
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Dtpresc::Div2
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Dtpresc::Div4
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Dtpresc::Div8
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Dtpresc::Div16
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Dtpresc::Div32
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Dtpresc::Div64
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Dtpresc::Div128
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Dtpresc::Div256
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Dtpresc::Div512
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Dtpresc::Div1024
    }
}
#[doc = "Field `DTPRESC` writer - DTI Prescaler Setting"]
pub type DtprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dtpresc>;
impl<'a, REG> DtprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpresc::Div1024)
    }
}
#[doc = "Field `DTRISET` reader - DTI Rise-time"]
pub type DtrisetR = crate::FieldReader;
#[doc = "Field `DTRISET` writer - DTI Rise-time"]
pub type DtrisetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DTFALLT` reader - DTI Fall-time"]
pub type DtfalltR = crate::FieldReader;
#[doc = "Field `DTFALLT` writer - DTI Fall-time"]
pub type DtfalltW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&self) -> DtprescR {
        DtprescR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&self) -> DtrisetR {
        DtrisetR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&self) -> DtfalltR {
        DtfalltR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&mut self) -> DtprescW<'_, DttimeSpec> {
        DtprescW::new(self, 0)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&mut self) -> DtrisetW<'_, DttimeSpec> {
        DtrisetW::new(self, 8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&mut self) -> DtfalltW<'_, DttimeSpec> {
        DtfalltW::new(self, 16)
    }
}
#[doc = "DTI Time Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dttime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dttime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DttimeSpec;
impl crate::RegisterSpec for DttimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttime::R`](R) reader structure"]
impl crate::Readable for DttimeSpec {}
#[doc = "`write(|w| ..)` method takes [`dttime::W`](W) writer structure"]
impl crate::Writable for DttimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTTIME to value 0"]
impl crate::Resettable for DttimeSpec {}
