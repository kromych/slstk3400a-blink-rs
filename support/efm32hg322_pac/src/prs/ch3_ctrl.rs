#[doc = "Register `CH3_CTRL` reader"]
pub type R = crate::R<CH3_CTRL_SPEC>;
#[doc = "Register `CH3_CTRL` writer"]
pub type W = crate::W<CH3_CTRL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Voltage Comparator"]
    VCMP = 1,
    #[doc = "2: Analog Comparator 0"]
    ACMP0 = 2,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 17,
    #[doc = "28: Timer 0"]
    TIMER0 = 28,
    #[doc = "29: Timer 1"]
    TIMER1 = 29,
    #[doc = "30: Timer 2"]
    TIMER2 = 30,
    #[doc = "36: Universal Serial Bus Interface"]
    USB = 36,
    #[doc = "40: Real-Time Counter"]
    RTC = 40,
    #[doc = "48: General purpose Input/Output"]
    GPIOL = 48,
    #[doc = "49: General purpose Input/Output"]
    GPIOH = 49,
    #[doc = "54: Pulse Counter 0"]
    PCNT0 = 54,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOURCESEL_A {
    type Ux = u8;
}
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            1 => Some(SOURCESEL_A::VCMP),
            2 => Some(SOURCESEL_A::ACMP0),
            8 => Some(SOURCESEL_A::ADC0),
            16 => Some(SOURCESEL_A::USART0),
            17 => Some(SOURCESEL_A::USART1),
            28 => Some(SOURCESEL_A::TIMER0),
            29 => Some(SOURCESEL_A::TIMER1),
            30 => Some(SOURCESEL_A::TIMER2),
            36 => Some(SOURCESEL_A::USB),
            40 => Some(SOURCESEL_A::RTC),
            48 => Some(SOURCESEL_A::GPIOL),
            49 => Some(SOURCESEL_A::GPIOH),
            54 => Some(SOURCESEL_A::PCNT0),
            _ => None,
        }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Voltage Comparator"]
    #[inline(always)]
    pub fn is_vcmp(&self) -> bool {
        *self == SOURCESEL_A::VCMP
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL_A::ACMP0
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESEL_A::TIMER2
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == SOURCESEL_A::USB
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == SOURCESEL_A::RTC
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL_A::GPIOL
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL_A::GPIOH
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL_A::PCNT0
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, SOURCESEL_A>;
impl<'a, REG> SOURCESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Voltage Comparator"]
    #[inline(always)]
    pub fn vcmp(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::VCMP)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER2)
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USB)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::RTC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PCNT0)
    }
}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EDSEL_R = crate::FieldReader<EDSEL_A>;
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF = 0,
    #[doc = "1: A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE = 1,
    #[doc = "2: A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE = 2,
    #[doc = "3: A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES = 3,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDSEL_A {
    type Ux = u8;
}
impl EDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL_A::OFF
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL_A::POSEDGE
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL_A::NEGEDGE
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL_A::BOTHEDGES
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EDSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EDSEL_A>;
impl<'a, REG> EDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::BOTHEDGES)
    }
}
#[doc = "Field `ASYNC` reader - Asynchronous reflex"]
pub type ASYNC_R = crate::BitReader;
#[doc = "Field `ASYNC` writer - Asynchronous reflex"]
pub type ASYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&self) -> ASYNC_R {
        ASYNC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<CH3_CTRL_SPEC> {
        SIGSEL_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<CH3_CTRL_SPEC> {
        SOURCESEL_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EDSEL_W<CH3_CTRL_SPEC> {
        EDSEL_W::new(self, 24)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    #[must_use]
    pub fn async_(&mut self) -> ASYNC_W<CH3_CTRL_SPEC> {
        ASYNC_W::new(self, 28)
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
#[doc = "Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_CTRL_SPEC;
impl crate::RegisterSpec for CH3_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_ctrl::R`](R) reader structure"]
impl crate::Readable for CH3_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3_ctrl::W`](W) writer structure"]
impl crate::Writable for CH3_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3_CTRL to value 0"]
impl crate::Resettable for CH3_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
