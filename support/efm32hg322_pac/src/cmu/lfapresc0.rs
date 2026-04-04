#[doc = "Register `LFAPRESC0` reader"]
pub type R = crate::R<Lfapresc0Spec>;
#[doc = "Register `LFAPRESC0` writer"]
pub type W = crate::W<Lfapresc0Spec>;
#[doc = "Real-Time Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtc {
    #[doc = "0: LFACLKRTC = LFACLK"]
    Div1 = 0,
    #[doc = "1: LFACLKRTC = LFACLK/2"]
    Div2 = 1,
    #[doc = "2: LFACLKRTC = LFACLK/4"]
    Div4 = 2,
    #[doc = "3: LFACLKRTC = LFACLK/8"]
    Div8 = 3,
    #[doc = "4: LFACLKRTC = LFACLK/16"]
    Div16 = 4,
    #[doc = "5: LFACLKRTC = LFACLK/32"]
    Div32 = 5,
    #[doc = "6: LFACLKRTC = LFACLK/64"]
    Div64 = 6,
    #[doc = "7: LFACLKRTC = LFACLK/128"]
    Div128 = 7,
    #[doc = "8: LFACLKRTC = LFACLK/256"]
    Div256 = 8,
    #[doc = "9: LFACLKRTC = LFACLK/512"]
    Div512 = 9,
    #[doc = "10: LFACLKRTC = LFACLK/1024"]
    Div1024 = 10,
    #[doc = "11: LFACLKRTC = LFACLK/2048"]
    Div2048 = 11,
    #[doc = "12: LFACLKRTC = LFACLK/4096"]
    Div4096 = 12,
    #[doc = "13: LFACLKRTC = LFACLK/8192"]
    Div8192 = 13,
    #[doc = "14: LFACLKRTC = LFACLK/16384"]
    Div16384 = 14,
    #[doc = "15: LFACLKRTC = LFACLK/32768"]
    Div32768 = 15,
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtc {
    type Ux = u8;
}
impl crate::IsEnum for Rtc {}
#[doc = "Field `RTC` reader - Real-Time Counter Prescaler"]
pub type RtcR = crate::FieldReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            0 => Rtc::Div1,
            1 => Rtc::Div2,
            2 => Rtc::Div4,
            3 => Rtc::Div8,
            4 => Rtc::Div16,
            5 => Rtc::Div32,
            6 => Rtc::Div64,
            7 => Rtc::Div128,
            8 => Rtc::Div256,
            9 => Rtc::Div512,
            10 => Rtc::Div1024,
            11 => Rtc::Div2048,
            12 => Rtc::Div4096,
            13 => Rtc::Div8192,
            14 => Rtc::Div16384,
            15 => Rtc::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Rtc::Div1
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Rtc::Div2
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Rtc::Div4
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Rtc::Div8
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Rtc::Div16
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Rtc::Div32
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Rtc::Div64
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Rtc::Div128
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Rtc::Div256
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Rtc::Div512
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Rtc::Div1024
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Rtc::Div2048
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Rtc::Div4096
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Rtc::Div8192
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Rtc::Div16384
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == Rtc::Div32768
    }
}
#[doc = "Field `RTC` writer - Real-Time Counter Prescaler"]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rtc, crate::Safe>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div1)
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div2)
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div4)
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div8)
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div16)
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div32)
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div64)
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div128)
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div256)
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div512)
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div1024)
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div2048)
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div4096)
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div8192)
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div16384)
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div32768)
    }
}
impl R {
    #[doc = "Bits 0:3 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Lfapresc0Spec> {
        RtcW::new(self, 0)
    }
}
#[doc = "Low Frequency A Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfapresc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfapresc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfapresc0Spec;
impl crate::RegisterSpec for Lfapresc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfapresc0::R`](R) reader structure"]
impl crate::Readable for Lfapresc0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfapresc0::W`](W) writer structure"]
impl crate::Writable for Lfapresc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFAPRESC0 to value 0"]
impl crate::Resettable for Lfapresc0Spec {}
